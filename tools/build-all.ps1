param(
    [switch]$Release
)

$ErrorActionPreference = "Stop"
$projectRoot = Split-Path -Parent $PSScriptRoot

Push-Location $projectRoot
try {
    Write-Host "Building Windows installer..." -ForegroundColor Cyan
    & "$PSScriptRoot\package-windows.ps1"

    Write-Host "Building Android APK..." -ForegroundColor Cyan
    if ($Release) {
        & "$PSScriptRoot\package-android.ps1"
    } else {
        & "$PSScriptRoot\package-android.ps1" -AllowDebugSigning
    }

    Write-Host "Build complete. Open the dist folder for the finished files." -ForegroundColor Green
} finally {
    Pop-Location
}
