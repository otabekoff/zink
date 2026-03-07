import * as vscode from "vscode";

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

let replTerminal: vscode.Terminal | undefined;
let outputChannel: vscode.OutputChannel | undefined;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/** Resolve the path to the `zink` interpreter binary. */
function getInterpreterPath(): string {
  const config = vscode.workspace.getConfiguration("zink");
  const custom = config.get<string>("interpreterPath", "").trim();
  return custom || "zink";
}

/** Return the shared Zink output channel (created once). */
function getOutputChannel(): vscode.OutputChannel {
  if (!outputChannel) {
    outputChannel = vscode.window.createOutputChannel("Zink");
  }
  return outputChannel;
}

/** Get or create a terminal named "Zink" for running files. */
function getRunTerminal(): vscode.Terminal {
  const existing = vscode.window.terminals.find((t) => t.name === "Zink");
  if (existing) {
    return existing;
  }
  return vscode.window.createTerminal("Zink");
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/** Run the currently open `.zink` file. */
async function runFile(): Promise<void> {
  const editor = vscode.window.activeTextEditor;
  if (!editor) {
    vscode.window.showWarningMessage("No active editor — open a .zink file first.");
    return;
  }

  // Save before running
  if (editor.document.isDirty) {
    await editor.document.save();
  }

  const filePath = editor.document.fileName;
  const interpreter = getInterpreterPath();
  const config = vscode.workspace.getConfiguration("zink");
  const useTerminal = config.get<boolean>("runInIntegratedTerminal", true);
  const clearFirst = config.get<boolean>("clearTerminalBeforeRun", true);

  if (useTerminal) {
    const terminal = getRunTerminal();
    terminal.show(true);
    if (clearFirst) {
      terminal.sendText("clear", true);
    }
    terminal.sendText(`${interpreter} "${filePath}"`, true);
  } else {
    const out = getOutputChannel();
    out.show(true);
    out.clear();
    out.appendLine(`> ${interpreter} "${filePath}"\n`);

    const cp = await import("child_process");
    const child = cp.spawn(interpreter, [filePath], {
      cwd: vscode.workspace.workspaceFolders?.[0]?.uri.fsPath,
      shell: true,
    });

    child.stdout?.on("data", (data: Buffer) => out.append(data.toString()));
    child.stderr?.on("data", (data: Buffer) => out.append(data.toString()));
    child.on("close", (code: number | null) => {
      out.appendLine(`\n[Process exited with code ${code ?? "unknown"}]`);
    });
  }
}

/** Run the selected text (or the current line) as a Zink snippet via stdin. */
async function runSelection(): Promise<void> {
  const editor = vscode.window.activeTextEditor;
  if (!editor) {
    return;
  }

  const selection = editor.selection;
  const text = selection.isEmpty
    ? editor.document.lineAt(selection.active.line).text
    : editor.document.getText(selection);

  if (!text.trim()) {
    return;
  }

  const interpreter = getInterpreterPath();
  const terminal = getRunTerminal();
  terminal.show(true);

  // Pipe the text through stdin using echo / printf
  const escaped = text.replace(/"/g, '\\"').replace(/\n/g, "\\n");
  terminal.sendText(`echo "${escaped}" | ${interpreter} -`, true);
}

/** Open (or reveal) the Zink REPL in the integrated terminal. */
function openRepl(): void {
  if (replTerminal) {
    // Check if the terminal is still alive
    const alive = vscode.window.terminals.includes(replTerminal);
    if (alive) {
      replTerminal.show();
      return;
    }
    replTerminal = undefined;
  }

  const interpreter = getInterpreterPath();
  replTerminal = vscode.window.createTerminal({
    name: "Zink REPL",
    shellPath: interpreter,
    shellArgs: ["--repl"],
  });
  replTerminal.show();
}

/** Dispose and recreate the REPL terminal. */
function restartRepl(): void {
  if (replTerminal) {
    replTerminal.dispose();
    replTerminal = undefined;
  }
  openRepl();
}

/** Run `cargo build --release` inside the `lang/` directory. */
async function buildBinary(): Promise<void> {
  // Try to find the lang/ folder relative to the workspace
  const folders = vscode.workspace.workspaceFolders;
  if (!folders || folders.length === 0) {
    vscode.window.showErrorMessage("No workspace folder open.");
    return;
  }

  const root = folders[0].uri.fsPath;
  const langDir = vscode.Uri.joinPath(vscode.Uri.file(root), "lang").fsPath;

  const terminal = vscode.window.createTerminal({
    name: "Zink Build",
    cwd: langDir,
  });
  terminal.show(true);
  terminal.sendText("cargo build --release", true);
}

// ---------------------------------------------------------------------------
// Activation / Deactivation
// ---------------------------------------------------------------------------

export function activate(context: vscode.ExtensionContext): void {
  // Register commands
  context.subscriptions.push(
    vscode.commands.registerCommand("zink.runFile", runFile),
    vscode.commands.registerCommand("zink.runSelection", runSelection),
    vscode.commands.registerCommand("zink.openRepl", openRepl),
    vscode.commands.registerCommand("zink.restartRepl", restartRepl),
    vscode.commands.registerCommand("zink.buildBinary", buildBinary),
  );

  // Listen for terminal close so we can clean up REPL reference
  context.subscriptions.push(
    vscode.window.onDidCloseTerminal((t) => {
      if (t === replTerminal) {
        replTerminal = undefined;
      }
    }),
  );

  // Show activation message only in dev mode
  if (context.extensionMode === vscode.ExtensionMode.Development) {
    const out = getOutputChannel();
    out.appendLine("Zink Language extension activated.");
  }
}

export function deactivate(): void {
  replTerminal?.dispose();
  outputChannel?.dispose();
  replTerminal = undefined;
  outputChannel = undefined;
}
