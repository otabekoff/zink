// src/interpreter.rs — Zink Language Tree-Walk Interpreter

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::parser::{BinOp, Expr, Stmt, UnaryOp};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// ── Runtime Values ───────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
    Array(Rc<RefCell<Vec<Value>>>),
    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
        closure: Env,
    },
    NativeFn(String),
}

impl Value {
    pub fn display(&self) -> String {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 && n.abs() < 1e15 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::Str(s) => s.clone(),
            Value::Bool(b) => {
                if *b {
                    "true".into()
                } else {
                    "false".into()
                }
            }
            Value::Nil => "null".into(),
            Value::Array(a) => {
                let items: Vec<String> = a.borrow().iter().map(|v| v.display()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Function { .. } => "<function>".into(),
            Value::NativeFn(name) => format!("<builtin:{name}>"),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(false) | Value::Nil => false,
            Value::Number(n) if *n == 0.0 => false,
            Value::Str(s) if s.is_empty() => false,
            Value::Array(a) if a.borrow().is_empty() => false,
            _ => true,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }
}

// ── Environment (scope chain) ────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Env(Rc<RefCell<EnvInner>>);

#[derive(Debug)]
struct EnvInner {
    vars: HashMap<String, Value>,
    parent: Option<Env>,
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

impl Env {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(EnvInner {
            vars: HashMap::new(),
            parent: None,
        })))
    }

    pub fn child(parent: &Env) -> Self {
        Self(Rc::new(RefCell::new(EnvInner {
            vars: HashMap::new(),
            parent: Some(parent.clone()),
        })))
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        let inner = self.0.borrow();
        if let Some(v) = inner.vars.get(name) {
            return Some(v.clone());
        }
        inner.parent.as_ref().and_then(|p| p.get(name))
    }

    pub fn set(&self, name: &str, value: Value) -> bool {
        let mut inner = self.0.borrow_mut();
        if inner.vars.contains_key(name) {
            inner.vars.insert(name.to_string(), value);
            return true;
        }
        if let Some(parent) = &inner.parent {
            return parent.set(name, value);
        }
        false
    }

    pub fn define(&self, name: &str, value: Value) {
        self.0.borrow_mut().vars.insert(name.to_string(), value);
    }
}

// ── Runtime Error ────────────────────────────────────────────────

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub line: usize,
}

macro_rules! rerr {
    ($line:expr, $($arg:tt)*) => {
        RuntimeError { message: format!($($arg)*), line: $line }
    };
}

// ── Control flow signals ─────────────────────────────────────────

enum Signal {
    Return(Value),
    None,
}

// ── Interpreter ──────────────────────────────────────────────────

pub struct Interpreter {
    pub global: Env,
    steps: usize,
    max_steps: usize,
    output: Vec<String>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let global = Env::new();
        let interp = Self {
            global: global.clone(),
            steps: 0,
            max_steps: 500_000,
            output: Vec::new(),
        };
        interp.register_builtins();
        interp
    }

    fn register_builtins(&self) {
        let builtins = [
            "len", "push", "pop", "str", "num", "floor", "ceil", "round", "abs", "sqrt", "pow",
            "max", "min", "random", "range", "type", "contains", "join", "split", "upper", "lower",
            "trim", "slice", "reverse", "sort", "map", "filter", "reduce", "keys",
        ];
        for name in builtins {
            self.global.define(name, Value::NativeFn(name.to_string()));
        }
    }

    pub fn run(&mut self, stmts: &[Stmt]) -> Result<(), RuntimeError> {
        let env = self.global.clone();
        self.exec_block(stmts, &env)?;
        Ok(())
    }

    pub fn take_output(&mut self) -> Vec<String> {
        std::mem::take(&mut self.output)
    }

    fn exec_block(&mut self, stmts: &[Stmt], env: &Env) -> Result<Signal, RuntimeError> {
        for stmt in stmts {
            match self.exec_stmt(stmt, env)? {
                Signal::Return(v) => return Ok(Signal::Return(v)),
                Signal::None => {}
            }
        }
        Ok(Signal::None)
    }

    fn exec_stmt(&mut self, stmt: &Stmt, env: &Env) -> Result<Signal, RuntimeError> {
        self.steps += 1;
        if self.steps > self.max_steps {
            return Err(rerr!(
                0,
                "Execution limit exceeded — possible infinite loop"
            ));
        }

        match stmt {
            Stmt::VarDecl { name, value, .. } => {
                let v = self.eval_expr(value, env)?;
                env.define(name, v);
            }

            Stmt::Assign { target, value } => {
                let v = self.eval_expr(value, env)?;
                match target {
                    Expr::Ident(name) => {
                        if !env.set(name, v.clone()) {
                            env.define(name, v); // allow implicit define at top level
                        }
                    }
                    Expr::Index { obj, idx } => {
                        let arr = self.eval_expr(obj, env)?;
                        let i = self.eval_expr(idx, env)?;
                        if let (Value::Array(a), Value::Number(n)) = (arr, i) {
                            let idx = n as usize;
                            let mut borrow = a.borrow_mut();
                            if idx < borrow.len() {
                                borrow[idx] = v;
                            }
                        }
                    }
                    _ => return Err(rerr!(0, "Invalid assignment target")),
                }
            }

            Stmt::FnDecl {
                name, params, body, ..
            } => {
                env.define(
                    name,
                    Value::Function {
                        params: params.clone(),
                        body: body.clone(),
                        closure: env.clone(),
                    },
                );
            }

            Stmt::Return { value, .. } => {
                let v = match value {
                    Some(e) => self.eval_expr(e, env)?,
                    None => Value::Nil,
                };
                return Ok(Signal::Return(v));
            }

            Stmt::If {
                cond, then, else_, ..
            } => {
                let c = self.eval_expr(cond, env)?;
                if c.is_truthy() {
                    let child = Env::child(env);
                    return self.exec_block(then, &child);
                } else if let Some(else_stmt) = else_ {
                    return self.exec_stmt(else_stmt, env);
                }
            }

            Stmt::While { cond, body, .. } => loop {
                self.steps += 1;
                if self.steps > self.max_steps {
                    return Err(rerr!(
                        0,
                        "Execution limit exceeded — possible infinite loop"
                    ));
                }
                let c = self.eval_expr(cond, env)?;
                if !c.is_truthy() {
                    break;
                }
                let child = Env::child(env);
                if let Signal::Return(v) = self.exec_block(body, &child)? {
                    return Ok(Signal::Return(v));
                }
            },

            Stmt::Loop { count, body, .. } => {
                let n = match self.eval_expr(count, env)? {
                    Value::Number(n) => n as usize,
                    _ => return Err(rerr!(0, "'loop N times' requires a number")),
                };
                for _ in 0..n {
                    self.steps += 1;
                    if self.steps > self.max_steps {
                        return Err(rerr!(0, "Execution limit exceeded"));
                    }
                    let child = Env::child(env);
                    if let Signal::Return(v) = self.exec_block(body, &child)? {
                        return Ok(Signal::Return(v));
                    }
                }
            }

            Stmt::Say { value, .. } => {
                let v = self.eval_expr(value, env)?;
                let line = self.interpolate(v, env)?;
                self.output.push(line);
            }

            Stmt::Expr { expr } => {
                self.eval_expr(expr, env)?;
            }
            Stmt::Block { body } => {
                let child = Env::child(env);
                return self.exec_block(body, &child);
            }
        }
        Ok(Signal::None)
    }

    fn eval_expr(&mut self, expr: &Expr, env: &Env) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Str(s) => Ok(Value::Str(s.clone())),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::Nil => Ok(Value::Nil),

            Expr::Ident(name) => env
                .get(name)
                .ok_or_else(|| rerr!(0, "Undefined variable '{name}'")),

            Expr::Array(items) => {
                let vals: Result<Vec<Value>, _> =
                    items.iter().map(|e| self.eval_expr(e, env)).collect();
                Ok(Value::Array(Rc::new(RefCell::new(vals?))))
            }

            Expr::BinOp { op, left, right } => {
                let l = self.eval_expr(left, env)?;
                let r = self.eval_expr(right, env)?;
                self.eval_binop(op, l, r)
            }

            Expr::UnaryOp { op, expr } => {
                let v = self.eval_expr(expr, env)?;
                match op {
                    UnaryOp::Neg => match v {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => Err(rerr!(0, "Unary minus requires a number")),
                    },
                    UnaryOp::Not => Ok(Value::Bool(!v.is_truthy())),
                }
            }

            Expr::Call { callee, args } => {
                let fn_val = self.eval_expr(callee, env)?;
                let arg_vals: Result<Vec<Value>, _> =
                    args.iter().map(|a| self.eval_expr(a, env)).collect();
                let arg_vals = arg_vals?;
                self.call_value(fn_val, arg_vals, env)
            }

            Expr::Index { obj, idx } => {
                let o = self.eval_expr(obj, env)?;
                let i = self.eval_expr(idx, env)?;
                match (o, i) {
                    (Value::Array(a), Value::Number(n)) => {
                        Ok(a.borrow().get(n as usize).cloned().unwrap_or(Value::Nil))
                    }
                    (Value::Str(s), Value::Number(n)) => Ok(s
                        .chars()
                        .nth(n as usize)
                        .map(|c| Value::Str(c.to_string()))
                        .unwrap_or(Value::Nil)),
                    _ => Err(rerr!(0, "Cannot index this type")),
                }
            }

            Expr::Prop { obj, prop } => {
                let o = self.eval_expr(obj, env)?;
                match (o, prop.as_str()) {
                    (Value::Array(a), "length") => Ok(Value::Number(a.borrow().len() as f64)),
                    (Value::Str(s), "length") => Ok(Value::Number(s.len() as f64)),
                    (_, p) => Err(rerr!(0, "No property '{p}'")),
                }
            }
            Expr::Lambda { params, body } => Ok(Value::Function {
                params: params.clone(),
                body: body.clone(),
                closure: env.clone(),
            }),
        }
    }

    fn eval_binop(&self, op: &BinOp, l: Value, r: Value) -> Result<Value, RuntimeError> {
        match op {
            BinOp::Add => match (l, r) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                (a, b) => Ok(Value::Str(a.display() + &b.display())),
            },
            BinOp::Sub => match (l, r) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                _ => Err(rerr!(0, "'-' requires numbers")),
            },
            BinOp::Mul => match (l, r) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                _ => Err(rerr!(0, "'*' requires numbers")),
            },
            BinOp::Div => match (l, r) {
                (Value::Number(_), Value::Number(0.0)) => Err(rerr!(0, "Division by zero")),
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
                _ => Err(rerr!(0, "'/' requires numbers")),
            },
            BinOp::Mod => match (l, r) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a % b)),
                _ => Err(rerr!(0, "'%' requires numbers")),
            },
            BinOp::Eq => Ok(Value::Bool(l == r)),
            BinOp::Neq => Ok(Value::Bool(l != r)),
            BinOp::Lt => self.cmp_num(l, r, |a, b| a < b),
            BinOp::Gt => self.cmp_num(l, r, |a, b| a > b),
            BinOp::Lte => self.cmp_num(l, r, |a, b| a <= b),
            BinOp::Gte => self.cmp_num(l, r, |a, b| a >= b),
            BinOp::And => Ok(Value::Bool(l.is_truthy() && r.is_truthy())),
            BinOp::Or => Ok(Value::Bool(l.is_truthy() || r.is_truthy())),
        }
    }

    fn cmp_num(
        &self,
        l: Value,
        r: Value,
        f: impl Fn(f64, f64) -> bool,
    ) -> Result<Value, RuntimeError> {
        match (l, r) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(f(a, b))),
            (Value::Str(a), Value::Str(b)) => Ok(Value::Bool(f(
                a.partial_cmp(&b).map(|o| o as i32 as f64).unwrap_or(0.0),
                0.0,
            ))),
            _ => Err(rerr!(0, "Comparison requires numbers")),
        }
    }

    fn call_value(
        &mut self,
        fn_val: Value,
        args: Vec<Value>,
        env: &Env,
    ) -> Result<Value, RuntimeError> {
        match fn_val {
            Value::Function {
                params,
                body,
                closure,
            } => {
                let fn_env = Env::child(&closure);
                for (i, p) in params.iter().enumerate() {
                    fn_env.define(p, args.get(i).cloned().unwrap_or(Value::Nil));
                }
                match self.exec_block(&body, &fn_env)? {
                    Signal::Return(v) => Ok(v),
                    Signal::None => Ok(Value::Nil),
                }
            }
            Value::NativeFn(name) => self.call_builtin(&name, args, env),
            _ => Err(rerr!(0, "Tried to call a non-function value")),
        }
    }

    fn call_builtin(
        &mut self,
        name: &str,
        args: Vec<Value>,
        env: &Env,
    ) -> Result<Value, RuntimeError> {
        let check_num = |v: &Value| match v {
            Value::Number(n) => Ok(*n),
            _ => Err(rerr!(0, "{name}() expects a number")),
        };

        match name {
            "len" => match args.first() {
                Some(Value::Array(a)) => Ok(Value::Number(a.borrow().len() as f64)),
                Some(Value::Str(s)) => Ok(Value::Number(s.len() as f64)),
                _ => Err(rerr!(0, "len() expects array or string")),
            },
            "push" => match args.as_slice() {
                [Value::Array(a), v] => {
                    a.borrow_mut().push(v.clone());
                    Ok(Value::Array(a.clone()))
                }
                _ => Err(rerr!(0, "push() expects (array, value)")),
            },
            "pop" => match args.first() {
                Some(Value::Array(a)) => Ok(a.borrow_mut().pop().unwrap_or(Value::Nil)),
                _ => Err(rerr!(0, "pop() expects array")),
            },
            "str" => Ok(Value::Str(
                args.first().map(|v| v.display()).unwrap_or_default(),
            )),
            "num" => match args.first() {
                Some(Value::Str(s)) => Ok(Value::Number(s.parse().unwrap_or(0.0))),
                Some(Value::Number(n)) => Ok(Value::Number(*n)),
                _ => Err(rerr!(0, "num() expects string or number")),
            },
            "floor" => Ok(Value::Number(
                check_num(args.first().unwrap_or(&Value::Nil))?.floor(),
            )),
            "ceil" => Ok(Value::Number(
                check_num(args.first().unwrap_or(&Value::Nil))?.ceil(),
            )),
            "round" => Ok(Value::Number(
                check_num(args.first().unwrap_or(&Value::Nil))?.round(),
            )),
            "abs" => Ok(Value::Number(
                check_num(args.first().unwrap_or(&Value::Nil))?.abs(),
            )),
            "sqrt" => Ok(Value::Number(
                check_num(args.first().unwrap_or(&Value::Nil))?.sqrt(),
            )),
            "pow" => match args.as_slice() {
                [Value::Number(b), Value::Number(e)] => Ok(Value::Number(b.powf(*e))),
                _ => Err(rerr!(0, "pow() expects (base, exp)")),
            },
            "max" => {
                let ns: Result<Vec<f64>, _> = args.iter().map(check_num).collect();
                Ok(Value::Number(
                    ns?.into_iter().fold(f64::NEG_INFINITY, f64::max),
                ))
            }
            "min" => {
                let ns: Result<Vec<f64>, _> = args.iter().map(check_num).collect();
                Ok(Value::Number(ns?.into_iter().fold(f64::INFINITY, f64::min)))
            }
            "random" => Ok(Value::Number(rand_f64())),
            "range" => {
                let start = match args.first() {
                    Some(Value::Number(n)) => *n as i64,
                    _ => return Err(rerr!(0, "range() expects numbers")),
                };
                let end = match args.get(1) {
                    Some(Value::Number(n)) => *n as i64,
                    _ => return Err(rerr!(0, "range() expects numbers")),
                };
                let step = match args.get(2) {
                    Some(Value::Number(n)) => *n as i64,
                    None => 1,
                    _ => 1,
                };
                let mut v = Vec::new();
                let mut i = start;
                while i < end {
                    v.push(Value::Number(i as f64));
                    i += step;
                }
                Ok(Value::Array(Rc::new(RefCell::new(v))))
            }
            "type" => Ok(Value::Str(
                match args.first() {
                    Some(Value::Number(_)) => "number",
                    Some(Value::Str(_)) => "string",
                    Some(Value::Bool(_)) => "bool",
                    Some(Value::Nil) => "null",
                    Some(Value::Array(_)) => "array",
                    Some(Value::Function { .. }) | Some(Value::NativeFn(_)) => "function",
                    None => "null",
                }
                .to_string(),
            )),
            "contains" => match args.as_slice() {
                [Value::Array(a), v] => Ok(Value::Bool(a.borrow().contains(v))),
                [Value::Str(s), Value::Str(needle)] => Ok(Value::Bool(s.contains(needle.as_str()))),
                _ => Err(rerr!(0, "contains() expects (collection, item)")),
            },
            "join" => match args.as_slice() {
                [Value::Array(a), Value::Str(sep)] => Ok(Value::Str(
                    a.borrow()
                        .iter()
                        .map(|v| v.display())
                        .collect::<Vec<_>>()
                        .join(sep),
                )),
                [Value::Array(a)] => Ok(Value::Str(
                    a.borrow()
                        .iter()
                        .map(|v| v.display())
                        .collect::<Vec<_>>()
                        .join(","),
                )),
                _ => Err(rerr!(0, "join() expects (array, separator)")),
            },
            "split" => match args.as_slice() {
                [Value::Str(s), Value::Str(sep)] => Ok(Value::Array(Rc::new(RefCell::new(
                    s.split(sep.as_str())
                        .map(|p| Value::Str(p.to_string()))
                        .collect(),
                )))),
                _ => Err(rerr!(0, "split() expects (string, separator)")),
            },
            "upper" => match args.first() {
                Some(Value::Str(s)) => Ok(Value::Str(s.to_uppercase())),
                _ => Err(rerr!(0, "upper() expects string")),
            },
            "lower" => match args.first() {
                Some(Value::Str(s)) => Ok(Value::Str(s.to_lowercase())),
                _ => Err(rerr!(0, "lower() expects string")),
            },
            "trim" => match args.first() {
                Some(Value::Str(s)) => Ok(Value::Str(s.trim().to_string())),
                _ => Err(rerr!(0, "trim() expects string")),
            },
            "slice" => match args.as_slice() {
                [Value::Array(a), Value::Number(s), Value::Number(e)] => {
                    let borrowed = a.borrow();
                    let start = *s as usize;
                    let end = (*e as usize).min(borrowed.len());
                    Ok(Value::Array(Rc::new(RefCell::new(
                        borrowed[start..end].to_vec(),
                    ))))
                }
                _ => Err(rerr!(0, "slice() expects (array, start, end)")),
            },
            "reverse" => match args.first() {
                Some(Value::Array(a)) => {
                    let mut v = a.borrow().clone();
                    v.reverse();
                    Ok(Value::Array(Rc::new(RefCell::new(v))))
                }
                _ => Err(rerr!(0, "reverse() expects array")),
            },
            "sort" => match args.first() {
                Some(Value::Array(a)) => {
                    let mut v = a.borrow().clone();
                    v.sort_by_key(|x| x.display());
                    Ok(Value::Array(Rc::new(RefCell::new(v))))
                }
                _ => Err(rerr!(0, "sort() expects array")),
            },
            "map" => match args.as_slice() {
                [Value::Array(a), fn_val] => {
                    let fn_val = fn_val.clone();
                    let items = a.borrow().clone();
                    let results: Result<Vec<Value>, _> = items
                        .iter()
                        .map(|v| self.call_value(fn_val.clone(), vec![v.clone()], env))
                        .collect();
                    Ok(Value::Array(Rc::new(RefCell::new(results?))))
                }
                _ => Err(rerr!(0, "map() expects (array, fn)")),
            },
            "filter" => match args.as_slice() {
                [Value::Array(a), fn_val] => {
                    let fn_val = fn_val.clone();
                    let items = a.borrow().clone();
                    let mut results = Vec::new();
                    for v in &items {
                        if self
                            .call_value(fn_val.clone(), vec![v.clone()], env)?
                            .is_truthy()
                        {
                            results.push(v.clone());
                        }
                    }
                    Ok(Value::Array(Rc::new(RefCell::new(results))))
                }
                _ => Err(rerr!(0, "filter() expects (array, fn)")),
            },
            "reduce" => match args.as_slice() {
                [Value::Array(a), fn_val, init] => {
                    let fn_val = fn_val.clone();
                    let items = a.borrow().clone();
                    let mut acc = init.clone();
                    for v in &items {
                        acc = self.call_value(fn_val.clone(), vec![acc, v.clone()], env)?;
                    }
                    Ok(acc)
                }
                _ => Err(rerr!(0, "reduce() expects (array, fn, initial)")),
            },
            other => Err(rerr!(0, "Unknown builtin '{other}'")),
        }
    }

    fn interpolate(&mut self, val: Value, env: &Env) -> Result<String, RuntimeError> {
        match val {
            Value::Str(s) => {
                let mut result = String::new();
                let chars: Vec<char> = s.chars().collect();
                let mut i = 0;
                while i < chars.len() {
                    if chars[i] == '{' {
                        i += 1;
                        let mut expr_str = String::new();
                        let mut brace_depth = 1;
                        while i < chars.len() && brace_depth > 0 {
                            let c = chars[i];
                            if c == '"' {
                                // Skip over string literals so braces inside them don't count
                                expr_str.push(c);
                                i += 1;
                                while i < chars.len() && chars[i] != '"' {
                                    if chars[i] == '\\' {
                                        expr_str.push(chars[i]);
                                        i += 1;
                                        if i < chars.len() {
                                            expr_str.push(chars[i]);
                                            i += 1;
                                        }
                                        continue;
                                    }
                                    expr_str.push(chars[i]);
                                    i += 1;
                                }
                                if i < chars.len() {
                                    expr_str.push(chars[i]); // closing "
                                    i += 1;
                                }
                                continue;
                            }
                            if c == '{' {
                                brace_depth += 1;
                            } else if c == '}' {
                                brace_depth -= 1;
                                if brace_depth == 0 {
                                    break;
                                }
                            }
                            expr_str.push(c);
                            i += 1;
                        }
                        i += 1; // skip closing '}'
                        // Parse and eval the expression inline
                        match self.eval_interpolation(&expr_str, env) {
                            Ok(v) => result.push_str(&v.display()),
                            Err(_) => {
                                result.push('{');
                                result.push_str(&expr_str);
                                result.push('}');
                            }
                        }
                    } else {
                        result.push(chars[i]);
                        i += 1;
                    }
                }
                Ok(result)
            }
            v => Ok(v.display()),
        }
    }

    fn eval_interpolation(&mut self, src: &str, env: &Env) -> Result<Value, RuntimeError> {
        let mut lexer = Lexer::new(src);
        let tokens = lexer
            .tokenize()
            .map_err(|e| rerr!(e.line, "{}", e.message))?;
        let mut parser = Parser::new(tokens);
        let expr = parser
            ._expr_pub()
            .map_err(|e| rerr!(e.line, "{}", e.message))?;
        self.eval_expr(&expr, env)
    }
}

// Simple xorshift PRNG (no dependencies needed)
fn rand_f64() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    static SEED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let mut s = SEED.load(std::sync::atomic::Ordering::Relaxed);
    if s == 0 {
        s = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos() as u64;
    }
    s ^= s << 13;
    s ^= s >> 7;
    s ^= s << 17;
    SEED.store(s, std::sync::atomic::Ordering::Relaxed);
    (s as f64) / (u64::MAX as f64)
}
