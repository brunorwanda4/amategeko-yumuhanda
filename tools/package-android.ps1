param(
    [switch]$Play,
    [switch]$AllowDebugSigning,
    [switch]$ExistingUserUpdate,
    [string]$SigningBaselineApk,
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$ExtraArgs
)

$ErrorActionPreference = "Stop"

if ($AllowDebugSigning -and $ExistingUserUpdate) {
    throw "Choose either -AllowDebugSigning or -ExistingUserUpdate, not both."
}
if ($Play -and $ExistingUserUpdate) {
    throw "-ExistingUserUpdate is only valid for the sideload APK."
}

foreach ($arg in $ExtraArgs) {
    if ($arg -eq "--allow-debug-signing") {
        $AllowDebugSigning = $true
    } else {
        throw "Unknown option: $arg"
    }
}

$metadata = cargo metadata --no-deps --format-version 1 | ConvertFrom-Json
$version = ($metadata.packages | Where-Object name -eq "mobile").version
if (-not $version) {
    throw "Workspace version not found"
}

$sdk = if ($env:ANDROID_HOME) { $env:ANDROID_HOME } else { "$env:LOCALAPPDATA\Android\Sdk" }
$ndk = Get-ChildItem -LiteralPath "$sdk\ndk" -Directory |
    Sort-Object Name -Descending |
    Select-Object -First 1 -ExpandProperty FullName
if (-not $ndk) {
    throw "Android NDK not found"
}
$env:ANDROID_HOME = $sdk
$env:ANDROID_NDK_HOME = $ndk

$buildTools = Get-ChildItem -LiteralPath "$sdk\build-tools" -Directory |
    Sort-Object { [version]$_.Name } -Descending |
    Select-Object -First 1
if (-not $buildTools) {
    throw "Android SDK build-tools not found"
}
$apkSigner = Join-Path $buildTools.FullName "apksigner.bat"
$zipAlign = Join-Path $buildTools.FullName "zipalign.exe"
if (-not (Test-Path -LiteralPath $apkSigner) -or -not (Test-Path -LiteralPath $zipAlign)) {
    throw "Android SDK apksigner and zipalign were not found under build-tools"
}

if (-not $Play -and -not $AllowDebugSigning) {
    if ([string]::IsNullOrWhiteSpace($SigningBaselineApk)) {
        throw "A publishable sideload APK requires -SigningBaselineApk pointing to an APK already installed by users. This prevents incompatible signing-key changes."
    }
    if (-not (Test-Path -LiteralPath $SigningBaselineApk -PathType Leaf)) {
        throw "Signing baseline APK not found: $SigningBaselineApk"
    }
    $SigningBaselineApk = (Resolve-Path -LiteralPath $SigningBaselineApk).Path
}

function Get-ApkSignerSha256 {
    param([Parameter(Mandatory = $true)][string]$ApkPath)

    $verification = & $apkSigner verify --print-certs $ApkPath 2>&1
    if ($LASTEXITCODE -ne 0) {
        throw "APK signature verification failed: $ApkPath"
    }
    foreach ($line in $verification) {
        if ([string]$line -match '^Signer #1 certificate SHA-256 digest:\s*([0-9a-fA-F]+)\s*$') {
            return $Matches[1].ToLowerInvariant()
        }
    }
    throw "Could not read the APK signer digest: $ApkPath"
}

$jniDir = "crates\mobile\android\app\src\main\jniLibs"
$jniAbiDir = Join-Path $jniDir "arm64-v8a"
if (Test-Path -LiteralPath $jniAbiDir) {
    Get-ChildItem -LiteralPath $jniAbiDir -Filter "*.so" -File | Remove-Item -Force
}
$cargoArgs = @("ndk", "-t", "arm64-v8a", "-o", $jniDir, "--platform", "26", "build", "-p", "mobile", "--release")
if ($Play) {
    $cargoArgs += "--no-default-features"
}
& cargo @cargoArgs
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}
Get-ChildItem -LiteralPath $jniAbiDir -Filter "*.so" -File |
    Where-Object Name -ne "libgpui_mobile_app.so" |
    Remove-Item -Force

$flavor = if ($Play) { "play" } else { "sideload" }
$gradleTask = if ($Play) { "assemblePlayRelease" } else { "assembleSideloadRelease" }
$source = "crates\mobile\android\app\build\outputs\apk\$flavor\release\app-$flavor-release.apk"
Remove-Item -LiteralPath $source -Force -ErrorAction SilentlyContinue

Push-Location "crates\mobile\android"
try {
    $gradleArgs = @($gradleTask)
    if ($AllowDebugSigning -or $ExistingUserUpdate) {
        $gradleArgs += "-PallowDebugSigning=true"
    }
    & .\gradlew.bat @gradleArgs
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
} finally {
    Pop-Location
}

if (-not (Test-Path -LiteralPath $source)) {
    throw "Signed APK not found at: $source"
}

New-Item -ItemType Directory -Force -Path "dist" | Out-Null
Get-ChildItem -LiteralPath "dist" -Filter "*unsigned*.apk" -File -ErrorAction SilentlyContinue |
    Remove-Item -Force

$suffix = if ($AllowDebugSigning) { "android-arm64-debugsigned" } else { "android-arm64" }
$destination = "dist\amategeko-yumuhanda-$version-$suffix.apk"
$candidate = "dist\.amategeko-yumuhanda-$version-$suffix.candidate.apk"
Remove-Item -LiteralPath $candidate, $destination -Force -ErrorAction SilentlyContinue
Copy-Item -LiteralPath $source -Destination $candidate

& $apkSigner verify --verbose $candidate
if ($LASTEXITCODE -ne 0) {
    Remove-Item -LiteralPath $candidate -Force -ErrorAction SilentlyContinue
    throw "APK signature verification failed"
}

if (-not $Play -and -not $AllowDebugSigning) {
    $baselineSigner = Get-ApkSignerSha256 -ApkPath $SigningBaselineApk
    $candidateSigner = Get-ApkSignerSha256 -ApkPath $candidate
    if ($baselineSigner -ne $candidateSigner) {
        Remove-Item -LiteralPath $candidate -Force -ErrorAction SilentlyContinue
        throw "APK signing key does not match the baseline APK. Configure the exact key used for the installed release; Android will reject this APK as an update."
    }
}

& $zipAlign -c -P 4 -v 4 $candidate
if ($LASTEXITCODE -ne 0) {
    Remove-Item -LiteralPath $candidate -Force -ErrorAction SilentlyContinue
    throw "APK alignment verification failed"
}

Move-Item -LiteralPath $candidate -Destination $destination -Force
Write-Output $destination
