$ErrorActionPreference = "Stop"

$metadata = cargo metadata --no-deps --format-version 1 | ConvertFrom-Json
$version = ($metadata.packages | Where-Object name -eq "desktop").version
if (-not $version) {
    throw "Workspace version not found"
}

cargo build -p desktop --release
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

$isccCandidates = @(
    "$env:LOCALAPPDATA\Programs\Inno Setup 6\ISCC.exe",
    "${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe",
    "$env:ProgramFiles\Inno Setup 6\ISCC.exe"
)
$iscc = $isccCandidates | Where-Object { Test-Path -LiteralPath $_ } | Select-Object -First 1
if (-not $iscc) {
    throw "Inno Setup 6 is not installed"
}

& $iscc "/DAppVersion=$version" "installer\windows.iss"
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

Write-Output "dist\amategeko-yumuhanda-$version-windows-setup.exe"
