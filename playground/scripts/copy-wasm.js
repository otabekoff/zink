// scripts/copy-wasm.js — Copy wasm-pack output into playground
import { cpSync, mkdirSync } from "fs";
import { resolve, dirname } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const pkg = resolve(__dirname, "../../lang/pkg");
const wasmDir = resolve(__dirname, "../src/wasm");
const publicDir = resolve(__dirname, "../public");

mkdirSync(wasmDir, { recursive: true });

cpSync(resolve(pkg, "zink_lang.js"), resolve(wasmDir, "zink_lang.js"));
cpSync(resolve(pkg, "zink_lang.d.ts"), resolve(wasmDir, "zink_lang.d.ts"));
cpSync(resolve(pkg, "zink_lang_bg.wasm.d.ts"), resolve(wasmDir, "zink_lang_bg.wasm.d.ts"));
cpSync(resolve(pkg, "zink_lang_bg.wasm"), resolve(publicDir, "zink_lang_bg.wasm"));

console.log("✓ WASM artifacts copied to playground");
