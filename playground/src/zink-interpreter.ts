// src/zink-interpreter.ts — WASM-backed Zink interpreter wrapper
//
// Replaces the hand-written JS interpreter with the real Rust
// interpreter compiled to WebAssembly via wasm-pack.

import __wbg_init, { run_zink } from "./wasm/zink_lang.js";

let ready = false;
let initPromise: Promise<void> | null = null;

/** Initialise the WASM module. Safe to call multiple times. */
export async function initZink(): Promise<void> {
  if (ready) return;
  if (!initPromise) {
    initPromise = __wbg_init({ module_or_path: "/zink_lang_bg.wasm" }).then(() => {
      ready = true;
    });
  }
  return initPromise;
}

/** Returns true once the WASM module has finished loading. */
export function isReady(): boolean {
  return ready;
}

/**
 * Run Zink source code and return output lines + errors.
 * The WASM module **must** be initialised first via `initZink()`.
 */
export function runZink(src: string): { out: string[]; errs: string[] } {
  if (!ready) {
    return { out: [], errs: ["Zink WASM module not initialised yet"] };
  }
  try {
    const result = run_zink(src) as { out?: string[]; errs?: string[] };
    return {
      out: Array.isArray(result?.out) ? result.out : [],
      errs: Array.isArray(result?.errs) ? result.errs : [],
    };
  } catch (e) {
    return { out: [], errs: [String(e)] };
  }
}
