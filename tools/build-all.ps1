param(
    [switch]$Release,
    [switch]$AllowDebugSigning,
    [switch]$ExistingUserUpdate,
    [string]$AndroidSigningBaselineApk
)

$ErrorActionPreference = "Stop"
$projectRoot = Split-Path -Parent $PSScriptRoot

if ($Release -and $AllowDebugSigning) {
    throw "Choose either -Release or -AllowDebugSigning, not both."
}
if ($ExistingUserUpdate -and -not $Release) {
    throw "-ExistingUserUpdate requires -Release and -AndroidSigningBaselineApk."
}
if (-not $Release -and -not $AllowDebugSigning) {
    throw "Choose -Release with -AndroidSigningBaselineApk for publishable packages, or -AllowDebugSigning for local testing."
}
if ($Release -and [string]::IsNullOrWhiteSpace($AndroidSigningBaselineApk)) {
    throw "-Release requires -AndroidSigningBaselineApk pointing to an APK already installed by users."
}

Push-Location $projectRoot
try {
    Write-Host "Building Windows installer..." -ForegroundColor Cyan
    & "$PSScriptRoot\package-windows.ps1"

    Write-Host "Building Android APK..." -ForegroundColor Cyan
    if ($Release) {
        & "$PSScriptRoot\package-android.ps1" -SigningBaselineApk $AndroidSigningBaselineApk -ExistingUserUpdate:$ExistingUserUpdate
    } else {
        & "$PSScriptRoot\package-android.ps1" -AllowDebugSigning
    }

    Write-Host "Build complete. Open the dist folder for the finished files." -ForegroundColor Green
} finally {
    Pop-Location
}
