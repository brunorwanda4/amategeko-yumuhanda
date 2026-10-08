param(
    [switch]$Portable
)

$ErrorActionPreference = "Stop"

& (Join-Path (Split-Path -Parent $PSScriptRoot) "installer\build.ps1") -Portable:$Portable
