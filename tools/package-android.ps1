param(
    [switch]$Play
)

$ErrorActionPreference = "Stop"
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

$jniDir = "crates\mobile\android\app\src\main\jniLibs"
$cargoArgs = @("ndk", "-t", "arm64-v8a", "-o", $jniDir, "--platform", "26", "build", "-p", "mobile", "--release")
if ($Play) {
    $cargoArgs += "--no-default-features"
}
& cargo @cargoArgs
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

$gradleTask = if ($Play) { "assemblePlayRelease" } else { "assembleSideloadRelease" }
Push-Location "crates\mobile\android"
try {
    & .\gradlew.bat $gradleTask
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
} finally {
    Pop-Location
}

$flavor = if ($Play) { "play" } else { "sideload" }
$source = "crates\mobile\android\app\build\outputs\apk\$flavor\release\app-$flavor-release-unsigned.apk"
$destination = "dist\amategeko-yumuhanda-$version-android-arm64-unsigned.apk"
New-Item -ItemType Directory -Force -Path "dist" | Out-Null
Copy-Item -LiteralPath $source -Destination $destination -Force
Write-Output $destination
