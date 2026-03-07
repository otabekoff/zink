# setup.ps1 — Build and install Zink (interpreter + VS Code extension)
#
#   Usage:
#     .\setup.ps1            Build & install everything
#     .\setup.ps1 lang       Build & install the Zink interpreter only
#     .\setup.ps1 extension  Build & install the VS Code extension only
#
#   The interpreter is installed via `cargo install --path lang`
#   which places the `zink` binary in ~/.cargo/bin (already on PATH).

param(
    [Parameter(Position = 0)]
    [ValidateSet("all", "lang", "extension")]
    [string]$Target = "all"
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $MyInvocation.MyCommand.Path

function Write-Step($msg) { Write-Host "`n⚡ $msg" -ForegroundColor Cyan }
function Write-Ok($msg)   { Write-Host "  ✓ $msg"  -ForegroundColor Green }
function Write-Err($msg)  { Write-Host "  ✗ $msg"  -ForegroundColor Red }

# ── Interpreter ──────────────────────────────────────────────────────
function Install-Lang {
    Write-Step "Building Zink interpreter (release)..."

    Push-Location "$Root\lang"
    try {
        cargo build --release
        if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }
        Write-Ok "Built lang\target\release\zink.exe"

        Write-Step "Installing Zink to cargo bin..."
        cargo install --path . --force
        if ($LASTEXITCODE -ne 0) { throw "cargo install failed" }

        $bin = (Get-Command zink -ErrorAction SilentlyContinue).Source
        if ($bin) {
            Write-Ok "zink installed at $bin"
            & zink --repl 2>$null & Stop-Process -Id $PID -ErrorAction SilentlyContinue
            Write-Host "  Run ``zink --repl`` or ``zink <file.zink>`` from anywhere." -ForegroundColor DarkGray
        } else {
            Write-Err "zink not found on PATH. Make sure ~/.cargo/bin is in your PATH."
        }
    } finally {
        Pop-Location
    }
}

# ── VS Code Extension ───────────────────────────────────────────────
function Install-Extension {
    Write-Step "Building VS Code extension..."

    Push-Location "$Root\extension"
    try {
        if (-not (Test-Path node_modules)) {
            Write-Host "  Installing npm dependencies..." -ForegroundColor DarkGray
            npm install
            if ($LASTEXITCODE -ne 0) { throw "npm install failed" }
        }

        npm run compile
        if ($LASTEXITCODE -ne 0) { throw "compile failed" }
        Write-Ok "TypeScript compiled to out/"

        npm run package
        if ($LASTEXITCODE -ne 0) { throw "vsce package failed" }

        $vsix = Get-ChildItem -Filter "*.vsix" | Sort-Object LastWriteTime -Descending | Select-Object -First 1
        if (-not $vsix) { throw "No .vsix file found" }
        Write-Ok "Packaged $($vsix.Name) ($([math]::Round($vsix.Length / 1KB)) KB)"

        Write-Step "Installing extension into VS Code..."
        code --install-extension $vsix.FullName --force
        if ($LASTEXITCODE -ne 0) { throw "code --install-extension failed" }
        Write-Ok "Extension installed — reload VS Code to activate."
    } finally {
        Pop-Location
    }
}

# ── Main ─────────────────────────────────────────────────────────────
Write-Host "╔══════════════════════════════════════╗" -ForegroundColor Yellow
Write-Host "║        Zink — Setup & Install        ║" -ForegroundColor Yellow
Write-Host "╚══════════════════════════════════════╝" -ForegroundColor Yellow

switch ($Target) {
    "lang"      { Install-Lang }
    "extension" { Install-Extension }
    default     { Install-Lang; Install-Extension }
}

Write-Host "`n🎉 Done!" -ForegroundColor Green
