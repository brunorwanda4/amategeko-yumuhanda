# Build script for crates/web-spike
param(
    [switch]$Release = $true
)

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path "$ScriptDir\.."
Set-Location $ProjectRoot

$ReleaseFlag = if ($Release) { "--release" } else { "" }
$BuildMode = if ($Release) { "release" } else { "debug" }

Write-Host "Building Web Spike (target: wasm32-unknown-unknown, mode: $BuildMode)..."

cargo +nightly rustc --package amategeko-web-spike --manifest-path crates/web-spike/Cargo.toml --target wasm32-unknown-unknown $ReleaseFlag -- -C link-arg=-zstack-size=8388608

$WasmPath = "$ProjectRoot\target\wasm32-unknown-unknown\$BuildMode\amategeko_web_spike.wasm"
if (-not (Test-Path $WasmPath)) {
    Write-Error "WASM binary not found at $WasmPath"
    exit 1
}

$OutDir = "$ProjectRoot\website\public\app-spike"
if (-not (Test-Path $OutDir)) {
    New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
}

Write-Host "Generating JS bindings with wasm-bindgen..."
wasm-bindgen $WasmPath --out-dir $OutDir --target web --no-typescript

# If wasm-opt is installed and available, optimize wasm
if (Get-Command wasm-opt -ErrorAction SilentlyContinue) {
    Write-Host "Optimizing with wasm-opt..."
    wasm-opt -O3 "$OutDir\amategeko_web_spike_bg.wasm" -o "$OutDir\amategeko_web_spike_bg.wasm"
}

Write-Host "Build complete! Artifacts in $OutDir"
