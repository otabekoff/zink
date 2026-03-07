// src/lib.rs — Zink Language Library
//
// Shared library used by:
//   - The CLI binary (src/main.rs)
//   - The WASM target (for playground and docs)

pub mod interpreter;
pub mod lexer;
pub mod parser;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

/// Run Zink source code and return collected output lines.
pub fn run_source(src: &str) -> Result<Vec<String>, String> {
    // 1. Lex
    let tokens = Lexer::new(src)
        .tokenize()
        .map_err(|e| format!("[Lex Error, line {}] {}", e.line, e.message))?;

    // 2. Parse
    let stmts = Parser::new(tokens)
        .parse()
        .map_err(|e| format!("[Parse Error, line {}] {}", e.line, e.message))?;

    // 3. Interpret
    let mut interp = Interpreter::new();
    interp.run(&stmts).map_err(|e| {
        if e.line > 0 {
            format!("[Runtime Error, line {}] {}", e.line, e.message)
        } else {
            format!("[Runtime Error] {}", e.message)
        }
    })?;

    Ok(interp.take_output())
}

// ── WASM Bindings ────────────────────────────────────────────────

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::run_source;
    use js_sys::{Array, Object, Reflect};
    use wasm_bindgen::prelude::*;

    /// Run Zink source code from JavaScript.
    /// Returns `{ out: string[], errs: string[] }`.
    #[wasm_bindgen]
    pub fn run_zink(src: &str) -> JsValue {
        let obj = Object::new();

        match run_source(src) {
            Ok(output) => {
                let arr = Array::new();
                for line in output {
                    arr.push(&JsValue::from_str(&line));
                }
                Reflect::set(&obj, &"out".into(), &arr).unwrap();
                Reflect::set(&obj, &"errs".into(), &Array::new()).unwrap();
            }
            Err(e) => {
                Reflect::set(&obj, &"out".into(), &Array::new()).unwrap();
                let errs = Array::new();
                errs.push(&JsValue::from_str(&e));
                Reflect::set(&obj, &"errs".into(), &errs).unwrap();
            }
        }

        obj.into()
    }
}
