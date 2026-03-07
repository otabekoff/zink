import * as assert from "assert";
import * as vscode from "vscode";

suite("Zink Extension", () => {
  test("Extension should be present", () => {
    const ext = vscode.extensions.getExtension("zink-lang.zink-lang");
    assert.ok(ext, "Extension not found");
  });

  test("Extension should register commands", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("zink.runFile"), "Missing zink.runFile");
    assert.ok(commands.includes("zink.runSelection"), "Missing zink.runSelection");
    assert.ok(commands.includes("zink.openRepl"), "Missing zink.openRepl");
    assert.ok(commands.includes("zink.restartRepl"), "Missing zink.restartRepl");
    assert.ok(commands.includes("zink.buildBinary"), "Missing zink.buildBinary");
  });

  test("Zink language should be registered", async () => {
    const langs = await vscode.languages.getLanguages();
    assert.ok(langs.includes("zink"), "Zink language not registered");
  });
});
