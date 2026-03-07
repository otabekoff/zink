import { useState, useRef, useEffect, useCallback, type KeyboardEvent } from "react";
import { initZink, runZink } from "./zink-interpreter.js";

/* ================================================================
   EXAMPLES
   ================================================================ */
const EXAMPLES = [
  {
    name: "Hello, World!",
    desc: "Your first Zink program",
    code: `# Welcome to Zink! ⚡
# Lines starting with # are comments

say "Hello, World!"
say "Zink is fast, simple, and fun!"

let name = "developer"
say "Welcome aboard, {name}!"`,
  },
  {
    name: "Variables & Math",
    desc: "Numbers, strings, arithmetic",
    code: `let x = 42
let y = 7

say "=== Arithmetic ==="
say "{x} + {y} = {x + y}"
say "{x} - {y} = {x - y}"
say "{x} * {y} = {x * y}"
say "{x} / {y} = {x / y}"
say "{x} % {y} = {x % y}"

let pi = 3.14159
let radius = 5
let area = pi * radius * radius
say ""
say "Circle area (r={radius}): {area}"`,
  },
  {
    name: "Functions",
    desc: "Defining and calling functions",
    code: `fn greet(name) {
  return "Hello, {name}! Welcome to Zink."
}

fn add(a, b) {
  return a + b
}

fn factorial(n) {
  if n <= 1 {
    return 1
  }
  return n * factorial(n - 1)
}

say greet("Alice")
say greet("Bob")
say ""
say "3 + 4 = {add(3, 4)}"
say ""
say "=== Factorials ==="
let i = 1
while i <= 8 {
  say "{i}! = {factorial(i)}"
  i = i + 1
}`,
  },
  {
    name: "FizzBuzz",
    desc: "Classic programming challenge",
    code: `# Classic FizzBuzz — 1 to 30
say "=== FizzBuzz ==="

let i = 1
while i <= 30 {
  if i % 15 == 0 {
    say "FizzBuzz"
  } else if i % 3 == 0 {
    say "Fizz"
  } else if i % 5 == 0 {
    say "Buzz"
  } else {
    say "{i}"
  }
  i = i + 1
}`,
  },
  {
    name: "Fibonacci",
    desc: "Sequence with recursion",
    code: `fn fib(n) {
  if n <= 1 {
    return n
  }
  return fib(n - 1) + fib(n - 2)
}

say "=== Fibonacci Sequence ==="
let i = 0
while i <= 12 {
  say "fib({i}) = {fib(i)}"
  i = i + 1
}`,
  },
  {
    name: "Arrays & Loops",
    desc: "Working with lists",
    code: `let fruits = ["apple", "banana", "cherry", "mango", "kiwi"]

say "=== Fruits ==="
let i = 0
while i < len(fruits) {
  say "{i + 1}. {fruits[i]}"
  i = i + 1
}

say ""
say "Total fruits: {len(fruits)}"

# Bubble sort
fn bubble_sort(arr) {
  let n = len(arr)
  let i = 0
  while i < n {
    let j = 0
    while j < n - i - 1 {
      if arr[j] > arr[j + 1] {
        let tmp = arr[j]
        arr[j] = arr[j + 1]
        arr[j + 1] = tmp
      }
      j = j + 1
    }
    i = i + 1
  }
  return arr
}

let nums = [64, 34, 25, 12, 22, 11, 90]
say ""
say "Before sort: {nums}"
let sorted = bubble_sort(nums)
say "After sort:  {sorted}"`,
  },
  {
    name: "Higher-Order Fns",
    desc: "map, filter, reduce",
    code: `let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

fn double(x) { return x * 2 }
fn is_even(x) { return x % 2 == 0 }
fn add(a, b) { return a + b }

let doubled = map(numbers, double)
say "Doubled: {doubled}"

let evens = filter(numbers, is_even)
say "Evens: {evens}"

let total = reduce(numbers, add, 0)
say "Sum: {total}"

# Piping them together
let result = reduce(filter(map(numbers, double), is_even), add, 0)
say "Sum of doubled evens: {result}"`,
  },
  {
    name: "Prime Numbers",
    desc: "Sieve & prime checker",
    code: `fn is_prime(n) {
  if n < 2 { return false }
  if n == 2 { return true }
  if n % 2 == 0 { return false }
  let i = 3
  while i * i <= n {
    if n % i == 0 { return false }
    i = i + 2
  }
  return true
}

say "=== Prime Numbers up to 100 ==="
let primes = []
let n = 2
while n <= 100 {
  if is_prime(n) {
    push(primes, n)
  }
  n = n + 1
}

say "Found {len(primes)} primes:"
say join(primes, ", ")`,
  },
];

/* ================================================================
   SYNTAX HIGHLIGHTER
   ================================================================ */
function highlight(code: string): string {
  const esc = code.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
  const lines = esc.split("\n");
  const pattern =
    /(#.*)$|("(?:[^"\\]|\\.)*")|(\b(?:let|fn|return|if|else|while|loop|times|say|and|or|not|null|true|false)\b)|(\b(?:len|push|pop|str|num|floor|ceil|round|abs|sqrt|pow|max|min|random|range|type|contains|join|split|upper|lower|trim|slice|reverse|sort|map|filter|reduce)\b(?=\s*\())|(\b[a-zA-Z_][a-zA-Z0-9_]*\b(?=\s*\())|(\b\d+\.?\d*\b)/gm;
  return lines
    .map((line) => {
      if (/^\s*#/.test(line))
        return `<span style="color:#4a6741;font-style:italic">${line}</span>`;
      return line.replace(
        pattern,
        (
          match: string,
          comment: string | undefined,
          str: string | undefined,
          kw: string | undefined,
          builtin: string | undefined,
          funcName: string | undefined,
          num: string | undefined,
        ) => {
          if (comment) return `<span style="color:#4a6741;font-style:italic">${comment}</span>`;
          if (str) return `<span style="color:#a8d48a">${str}</span>`;
          if (kw) return `<span style="color:#7eb8f7;font-weight:600">${kw}</span>`;
          if (builtin) return `<span style="color:#c792ea">${builtin}</span>`;
          if (funcName) return `<span style="color:#ffd580">${funcName}</span>`;
          if (num !== undefined && num !== "") return `<span style="color:#f78c6c">${num}</span>`;
          return match;
        },
      );
    })
    .join("\n");
}

/* ================================================================
   DOCS DATA
   ================================================================ */
const DOCS = [
  { title: "Variables", content: `let x = 42\nlet name = "Alice"\nlet flag = true\nlet nothing = null` },
  { title: "Say (Print)", content: `say "Hello!"\nsay "x = {x}"\nsay x + y` },
  {
    title: "If / Else",
    content: `if x > 0 {\n  say "positive"\n} else if x < 0 {\n  say "negative"\n} else {\n  say "zero"\n}`,
  },
  { title: "While Loop", content: `let i = 0\nwhile i < 10 {\n  say "{i}"\n  i = i + 1\n}` },
  { title: "Loop N Times", content: `loop 5 times {\n  say "hello"\n}` },
  { title: "Functions", content: `fn add(a, b) {\n  return a + b\n}\nsay add(3, 4)` },
  { title: "Arrays", content: `let arr = [1, 2, 3]\npush(arr, 4)\nsay len(arr)\nsay arr[0]` },
  {
    title: "String Interp.",
    content: `let name = "Zink"\nsay "Hello, {name}!"\nsay "2 + 2 = {2 + 2}"`,
  },
  {
    title: "Builtins",
    content: `len(arr)  push(arr, v)  pop(arr)\nstr(v)    num(s)       type(v)\nfloor(n)  ceil(n)      round(n)\nabs(n)    sqrt(n)      pow(b,e)\nmax(a,b)  min(a,b)     random()\nrange(s,e,step)        contains(c,x)\njoin(arr,sep)          split(str,sep)\nupper(s)  lower(s)     trim(s)\nslice(a,s,e)           reverse(arr)\nsort(arr) map(arr,fn)  filter(arr,fn)\nreduce(arr,fn,init)`,
  },
];

/* ================================================================
   MAIN IDE COMPONENT
   ================================================================ */
export default function ZinkIDE() {
  const [code, setCode] = useState(EXAMPLES[0].code);
  const [output, setOutput] = useState<string[]>([]);
  const [errors, setErrors] = useState<string[]>([]);
  const [running, setRunning] = useState(false);
  const [activeEx, setActiveEx] = useState(0);
  const [tab, setTab] = useState<"examples" | "docs">("examples");
  const [openDoc, setOpenDoc] = useState<number | null>(null);
  const [lineCount, setLineCount] = useState(1);
  const [outputTime, setOutputTime] = useState<string | null>(null);
  const [wasmReady, setWasmReady] = useState(false);
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const preRef = useRef<HTMLPreElement>(null);
  const outputRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    initZink().then(() => setWasmReady(true));
  }, []);

  useEffect(() => {
    setLineCount(code.split("\n").length);
  }, [code]);

  useEffect(() => {
    if (outputRef.current) outputRef.current.scrollTop = outputRef.current.scrollHeight;
  }, [output, errors]);

  const syncScroll = () => {
    if (textareaRef.current && preRef.current) {
      preRef.current.scrollTop = textareaRef.current.scrollTop;
      preRef.current.scrollLeft = textareaRef.current.scrollLeft;
    }
  };

  const run = useCallback(() => {
    if (!wasmReady) return;
    setRunning(true);
    setOutput([]);
    setErrors([]);
    const t0 = performance.now();
    setTimeout(() => {
      const { out, errs } = runZink(code);
      const elapsed = ((performance.now() - t0) / 1000).toFixed(3);
      setOutput(out);
      setErrors(errs);
      setOutputTime(elapsed);
      setRunning(false);
    }, 60);
  }, [code, wasmReady]);

  const loadExample = (idx: number) => {
    setActiveEx(idx);
    setCode(EXAMPLES[idx].code);
    setOutput([]);
    setErrors([]);
    setOutputTime(null);
  };

  const handleKey = (e: KeyboardEvent<HTMLTextAreaElement>) => {
    if ((e.metaKey || e.ctrlKey) && e.key === "Enter") {
      e.preventDefault();
      run();
    }
    if (e.key === "Tab") {
      e.preventDefault();
      const ta = e.currentTarget;
      const s = ta.selectionStart;
      const end = ta.selectionEnd;
      const newCode = code.slice(0, s) + "  " + code.slice(end);
      setCode(newCode);
      requestAnimationFrame(() => {
        ta.selectionStart = ta.selectionEnd = s + 2;
      });
    }
  };

  const lines = Array.from({ length: lineCount }, (_, i) => i + 1);

  return (
    <div
      style={{
        fontFamily: "-apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif",
        background: "#0a0a0b",
        color: "#c8c8cc",
        height: "100vh",
        display: "grid",
        gridTemplateRows: "46px 1fr",
        gridTemplateColumns: "240px 1fr",
        overflow: "hidden",
      }}
    >
      <style>{`
        @import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500&display=swap');
        * { box-sizing: border-box; margin: 0; padding: 0; }
        ::-webkit-scrollbar { width: 6px; height: 6px; }
        ::-webkit-scrollbar-track { background: transparent; }
        ::-webkit-scrollbar-thumb { background: #2a2a2e; border-radius: 3px; }
        .run-btn {
          background: #059669; color: #fff; border: none;
          padding: 6px 16px; border-radius: 5px; cursor: pointer;
          font-size: 13px; font-weight: 600; font-family: inherit;
          display: flex; align-items: center; gap: 6px;
          transition: background 0.12s;
        }
        .run-btn:hover { background: #047857; }
        .run-btn:disabled { opacity: 0.5; cursor: not-allowed; }
        .ex-btn {
          display: block; width: 100%; text-align: left;
          padding: 8px 14px; border: none; background: none;
          cursor: pointer; color: #777; font-size: 13px;
          border-left: 2px solid transparent;
          transition: color 0.1s, border-color 0.1s, background 0.1s;
          font-family: inherit; line-height: 1.4;
        }
        .ex-btn:hover { color: #bbb; background: #111113; }
        .ex-btn.active { color: #e0e0e2; border-left-color: #10b981; background: #111113; }
        .ex-desc { font-size: 11px; color: #4a4a50; margin-top: 2px; }
        .ex-btn.active .ex-desc { color: #606068; }
        .doc-row {
          display: block; width: 100%; text-align: left;
          padding: 6px 14px; cursor: pointer; color: #666;
          font-size: 12px; border: none; background: none;
          font-family: inherit; transition: color 0.1s;
        }
        .doc-row:hover { color: #aaa; }
        .doc-row.open { color: #10b981; }
        .doc-pre {
          margin: 2px 14px 8px; padding: 8px 10px;
          background: #111113; border-radius: 4px;
          font-size: 11px; color: #888; line-height: 1.5;
          font-family: 'JetBrains Mono', monospace;
          white-space: pre; overflow-x: auto;
        }
        .out-line { padding: 1px 0; color: #a8d4b4; line-height: 1.55; }
        .err-line { padding: 1px 0; color: #ef4444; line-height: 1.55; }
        textarea { resize: none; outline: none; border: none; }
      `}</style>

      {/* Header */}
      <div
        style={{
          gridColumn: "1 / -1",
          background: "#0e0e10",
          borderBottom: "1px solid #1c1c20",
          display: "flex",
          alignItems: "center",
          padding: "0 16px",
          justifyContent: "space-between",
        }}
      >
        <div style={{ display: "flex", alignItems: "baseline", gap: 8 }}>
          <span style={{ fontWeight: 700, fontSize: 16, color: "#e8e8ea", letterSpacing: "-0.3px" }}>
            zink
          </span>
          <span style={{ fontSize: 12, color: "#444" }}>playground</span>
        </div>
        <div style={{ display: "flex", alignItems: "center", gap: 14 }}>
          <span style={{ fontSize: 11, color: "#333" }}>{wasmReady ? "Ctrl+Enter to run" : "Loading WASM…"}</span>
          <button className="run-btn" onClick={run} disabled={running || !wasmReady}>
            {running ? "Running..." : "▶ Run"}
          </button>
        </div>
      </div>

      {/* Sidebar */}
      <div
        style={{
          gridRow: 2,
          background: "#0c0c0e",
          borderRight: "1px solid #1c1c20",
          overflowY: "auto",
          display: "flex",
          flexDirection: "column",
        }}
      >
        <div style={{ display: "flex", borderBottom: "1px solid #1c1c20" }}>
          {(["examples", "docs"] as const).map((t) => (
            <button
              key={t}
              onClick={() => setTab(t)}
              style={{
                flex: 1,
                padding: "9px 0",
                background: "none",
                border: "none",
                cursor: "pointer",
                fontSize: 12,
                fontFamily: "inherit",
                color: tab === t ? "#c8c8cc" : "#444",
                borderBottom: tab === t ? "1.5px solid #10b981" : "1.5px solid transparent",
                transition: "color 0.1s",
              }}
            >
              {t === "examples" ? "Examples" : "Docs"}
            </button>
          ))}
        </div>

        <div style={{ flex: 1, paddingTop: 6 }}>
          {tab === "examples"
            ? EXAMPLES.map((ex, i) => (
                <button
                  key={i}
                  className={`ex-btn${activeEx === i ? " active" : ""}`}
                  onClick={() => loadExample(i)}
                >
                  <div>{ex.name}</div>
                  <div className="ex-desc">{ex.desc}</div>
                </button>
              ))
            : DOCS.map((d, i) => (
                <div key={i}>
                  <button
                    className={`doc-row${openDoc === i ? " open" : ""}`}
                    onClick={() => setOpenDoc(openDoc === i ? null : i)}
                  >
                    {openDoc === i ? "▾" : "▸"} {d.title}
                  </button>
                  {openDoc === i && <pre className="doc-pre">{d.content}</pre>}
                </div>
              ))}
        </div>

        <div style={{ padding: "10px 14px", borderTop: "1px solid #1a1a1e", fontSize: 10, color: "#333" }}>
          MIT · Zink v0.1.0
        </div>
      </div>

      {/* Main panel */}
      <div
        style={{
          gridRow: 2,
          display: "grid",
          gridTemplateRows: "1fr 200px",
          overflow: "hidden",
        }}
      >
        {/* Editor */}
        <div style={{ position: "relative", overflow: "hidden", borderBottom: "1px solid #1c1c20" }}>
          <div
            style={{
              position: "absolute",
              top: 0,
              left: 0,
              right: 0,
              zIndex: 3,
              background: "#0e0e10",
              borderBottom: "1px solid #1c1c20",
              padding: "5px 14px",
              display: "flex",
              alignItems: "center",
            }}
          >
            <span style={{ fontSize: 11, color: "#555" }}>main.zink</span>
            <span style={{ fontSize: 10, color: "#333", marginLeft: "auto" }}>{lineCount} lines</span>
          </div>

          <div
            style={{
              position: "absolute",
              top: 28,
              left: 0,
              right: 0,
              bottom: 0,
              display: "flex",
              overflow: "hidden",
            }}
          >
            <div
              style={{
                width: 42,
                flexShrink: 0,
                background: "#08080a",
                borderRight: "1px solid #1a1a1e",
                padding: "10px 0",
                overflowY: "hidden",
                userSelect: "none",
              }}
            >
              {lines.map((n) => (
                <div
                  key={n}
                  style={{
                    height: "1.55em",
                    lineHeight: "1.55em",
                    fontSize: 12,
                    color: "#333",
                    textAlign: "right",
                    paddingRight: 8,
                    fontFamily: "'JetBrains Mono', monospace",
                  }}
                >
                  {n}
                </div>
              ))}
            </div>

            <div style={{ flex: 1, position: "relative", overflow: "hidden" }}>
              <pre
                ref={preRef}
                style={{
                  position: "absolute",
                  inset: 0,
                  margin: 0,
                  padding: "10px 12px",
                  fontSize: 13,
                  lineHeight: "1.55em",
                  fontFamily: "'JetBrains Mono', monospace",
                  whiteSpace: "pre",
                  overflow: "hidden",
                  pointerEvents: "none",
                  color: "#c8c8cc",
                  background: "transparent",
                }}
                dangerouslySetInnerHTML={{ __html: highlight(code) + "\n" }}
              />
              <textarea
                ref={textareaRef}
                value={code}
                onChange={(e) => setCode(e.target.value)}
                onKeyDown={handleKey}
                onScroll={syncScroll}
                spellCheck={false}
                style={{
                  position: "absolute",
                  inset: 0,
                  padding: "10px 12px",
                  fontSize: 13,
                  lineHeight: "1.55em",
                  fontFamily: "'JetBrains Mono', monospace",
                  whiteSpace: "pre",
                  background: "transparent",
                  color: "transparent",
                  caretColor: "#10b981",
                  width: "100%",
                  height: "100%",
                  overflow: "auto",
                }}
              />
            </div>
          </div>
        </div>

        {/* Output */}
        <div
          style={{
            background: "#08080a",
            display: "flex",
            flexDirection: "column",
            overflow: "hidden",
          }}
        >
          <div
            style={{
              background: "#0e0e10",
              borderBottom: "1px solid #1a1a1e",
              padding: "5px 14px",
              display: "flex",
              alignItems: "center",
              gap: 8,
            }}
          >
            <span style={{ fontSize: 11, color: "#555" }}>Output</span>
            {running && (
              <span style={{ fontSize: 10, color: "#10b981" }}>running...</span>
            )}
            {outputTime && !running && (
              <span style={{ fontSize: 10, color: "#444" }}>{outputTime}s</span>
            )}
            <button
              onClick={() => {
                setOutput([]);
                setErrors([]);
                setOutputTime(null);
              }}
              style={{
                marginLeft: "auto",
                background: "none",
                border: "none",
                cursor: "pointer",
                color: "#444",
                fontSize: 11,
                fontFamily: "inherit",
              }}
            >
              clear
            </button>
          </div>
          <div
            ref={outputRef}
            style={{
              flex: 1,
              overflowY: "auto",
              padding: "8px 14px",
              fontSize: 12,
              lineHeight: "1.55em",
              fontFamily: "'JetBrains Mono', monospace",
            }}
          >
            {output.length === 0 && errors.length === 0 && (
              <div style={{ color: "#333", fontSize: 11 }}>
                Press ▶ Run or Ctrl+Enter to execute...
              </div>
            )}
            {output.map((line, i) => (
              <div key={i} className="out-line">
                {line}
              </div>
            ))}
            {errors.map((err, i) => (
              <div key={i} className="err-line">
                {err}
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
