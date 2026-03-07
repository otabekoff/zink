// src/main.rs — Zink Language Entry Point
//
//   Usage:
//     zink <file.zink>         Run a Zink script
//     zink                     Start REPL (interactive mode)
//
//   Examples:
//     zink hello.zink
//     echo 'say "Hello!"' | zink -

use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use zink_lang::interpreter::Interpreter;
use zink_lang::lexer::Lexer;
use zink_lang::parser::Parser;
use zink_lang::run_source;

fn run_file(path: &str) {
    let src = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", path, e);
            std::process::exit(1);
        }
    };

    match run_source(&src) {
        Ok(lines) => {
            for line in lines {
                println!("{}", line);
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

fn count_brace_depth(line: &str) -> i32 {
    // Count net { vs } outside of strings and comments
    let mut depth: i32 = 0;
    let mut in_str = false;
    let mut escaped = false;
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if escaped {
            escaped = false;
            i += 1;
            continue;
        }
        if in_str {
            if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_str = false;
            }
        } else if c == '"' {
            in_str = true;
        } else if c == '#' {
            break; // rest of line is comment
        } else if c == '{' {
            depth += 1;
        } else if c == '}' {
            depth -= 1;
        }
        i += 1;
    }
    depth
}

fn repl() {
    println!("⚡ Zink v1.0 — REPL");
    println!("   Type code, press Enter. Opens a block with {{ and closes with }}.");
    println!("   Type 'exit' or Ctrl+C to quit.\n");

    let stdin = io::stdin();
    let mut buf = String::new();
    let mut depth: i32 = 0;
    let mut interp = Interpreter::new();

    loop {
        // Prompt depends on whether we're inside an open block
        if depth == 0 {
            print!("zink> ");
        } else {
            // Indent by current depth for readability
            print!("{:>width$}.. ", "", width = (depth as usize) * 2);
        }
        io::stdout().flush().ok();

        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => break, // EOF / Ctrl+D
            Err(e) => {
                eprintln!("Input error: {e}");
                break;
            }
            Ok(_) => {}
        }

        let trimmed = line.trim();
        if depth == 0 && (trimmed == "exit" || trimmed == "quit") {
            break;
        }

        // Track brace depth so we know when a block is complete
        depth += count_brace_depth(&line);
        if depth < 0 {
            depth = 0;
        } // safety: unmatched } resets

        buf.push_str(&line);

        // Only attempt to run when all opened braces are closed
        if depth > 0 {
            continue;
        }

        let src = buf.trim().to_string();
        buf.clear();

        if src.is_empty() {
            continue;
        }

        // Lex
        let tokens = match Lexer::new(&src).tokenize() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("[Lex Error] {}", e.message);
                continue;
            }
        };

        // Parse
        let stmts = match Parser::new(tokens).parse() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[Parse Error] {}", e.message);
                continue;
            }
        };

        // Run
        match interp.run(&stmts) {
            Ok(()) => {
                for out in interp.take_output() {
                    println!("{}", out);
                }
            }
            Err(e) => eprintln!("[Runtime Error] {}", e.message),
        }
    }

    println!("Goodbye! ✦");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        None | Some("--repl") => repl(),
        Some("-") => {
            // Read from stdin
            let src: String = io::stdin()
                .lock()
                .lines()
                .map(|l| l.unwrap_or_default() + "\n")
                .collect();
            match run_source(&src) {
                Ok(lines) => lines.iter().for_each(|l| println!("{l}")),
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(1);
                }
            }
        }
        Some(path) => run_file(path),
    }
}
