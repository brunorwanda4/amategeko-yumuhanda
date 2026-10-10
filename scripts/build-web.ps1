param(
    [switch]$Full = $false
)
$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path "$ScriptDir\.."
Set-Location $ProjectRoot

$QuestionSet = if ($Full -or ($env:QUESTION_SET -eq "full")) { "full" } else { "samples" }
$SamplesPath = "$ProjectRoot\website\content\samples\questions.json"

if ($QuestionSet -ne "full") {
    if (-not (Test-Path $SamplesPath)) {
        Write-Error "Error: website/content/samples/questions.json is missing or has fewer than 25 questions. Never invent questions."
        exit 1
    }
    $raw = Get-Content $SamplesPath -Raw | ConvertFrom-Json
    if ($raw.Count -lt 25) {
        Write-Error "Error: website/content/samples/questions.json has fewer than 25 questions. Never invent questions."
        exit 1
    }
}

$env:QUESTION_SET = $QuestionSet
$env:CARGO_WEB_BUILD = "1"

Write-Host "Building web crate (profile: web, QUESTION_SET: $QuestionSet)..."
cargo rustc --package web --target wasm32-unknown-unknown --profile web -- -C link-arg=-zstack-size=8388608

$WasmPath = "$ProjectRoot\target\wasm32-unknown-unknown\web\web.wasm"
if (-not (Test-Path $WasmPath)) {
    Write-Error "WASM binary not found at $WasmPath"
    exit 1
}

$OutDir = "$ProjectRoot\website\public\runtime"
if (-not (Test-Path $OutDir)) {
    New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
}

$TempDir = "$ProjectRoot\target\wasm32-unknown-unknown\web\bindgen_temp"
if (Test-Path $TempDir) {
    Remove-Item -Recurse -Force $TempDir
}
New-Item -ItemType Directory -Force -Path $TempDir | Out-Null

Write-Host "Generating JS bindings with wasm-bindgen..."
wasm-bindgen $WasmPath --out-dir $TempDir --target web --no-typescript

$WasmBg = "$TempDir\web_bg.wasm"
if (Get-Command wasm-opt -ErrorAction SilentlyContinue) {
    Write-Host "Optimizing with wasm-opt -Oz..."
    wasm-opt -Oz $WasmBg -o $WasmBg
} else {
    Write-Warning "wasm-opt not found in PATH; skipping wasm-opt optimization."
}

# Content hashing
$hash = (Get-FileHash -Algorithm SHA256 $WasmBg).Hash.Substring(0, 8).ToLower()
$HashedWasmName = "web_${hash}_bg.wasm"
$HashedJsName = "web_${hash}.js"

# Remove older hashed files
Get-ChildItem -Path $OutDir -Filter "web_*_bg.wasm" | Remove-Item -Force
Get-ChildItem -Path $OutDir -Filter "web_*.js" | Remove-Item -Force

# Copy files
Copy-Item $WasmBg "$OutDir\$HashedWasmName" -Force
$jsContent = Get-Content "$TempDir\web.js" -Raw
$jsContent = $jsContent.Replace("web_bg.wasm", $HashedWasmName)
Set-Content -Path "$OutDir\$HashedJsName" -Value $jsContent -NoNewline

# Build info
$wasmBytes = (Get-Item "$OutDir\$HashedWasmName").Length
$version = (Get-Content "$ProjectRoot\Cargo.toml" | Select-String -Pattern '^\s*version\s*=\s*"([^"]+)"' | Select-Object -First 1).Matches[0].Groups[1].Value
if (-not $version) {
    $version = "1.8.0"
}
$builtAt = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")

$buildInfo = [ordered]@{
    version = $version
    js = $HashedJsName
    wasm = $HashedWasmName
    wasmBytes = $wasmBytes
    questionSet = $QuestionSet
    builtAt = $builtAt
} | ConvertTo-Json

$written = $false
for ($i = 0; $i -lt 10; $i++) {
    try {
        [System.IO.File]::WriteAllText("$OutDir\build.json", $buildInfo)
        $written = $true
        break
    } catch {
        Start-Sleep -Milliseconds 300
    }
}
if (-not $written) {
    Set-Content -Path "$OutDir\build.json" -Value $buildInfo -Force
}

# Print sizes
$script = "const fs = require('node:fs'); const zlib = require('node:zlib'); const file = process.argv.slice(1).find(a => !a.startsWith('-') && fs.existsSync(a)); const buf = fs.readFileSync(file); const raw = buf.length; const gz = zlib.gzipSync(buf, { level: 9 }).length; const br = zlib.brotliCompressSync(buf, { params: { [zlib.constants.BROTLI_PARAM_QUALITY]: 11 } }).length; console.log('.wasm sizes for ' + file + ':'); console.log('  Raw:    ' + raw.toLocaleString() + ' bytes (' + (raw / 1024 / 1024).toFixed(2) + ' MB)'); console.log('  Gzip:   ' + gz.toLocaleString() + ' bytes (' + (gz / 1024 / 1024).toFixed(2) + ' MB)'); console.log('  Brotli: ' + br.toLocaleString() + ' bytes (' + (br / 1024 / 1024).toFixed(2) + ' MB)');"
bun -e $script "$OutDir\$HashedWasmName"

Write-Host "Web build complete! Artifacts written to $OutDir"
