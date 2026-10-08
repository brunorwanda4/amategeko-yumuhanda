param(
    [switch]$Portable
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$projectRoot = Split-Path -Parent $PSScriptRoot
$manifestPath = Join-Path $projectRoot "Cargo.toml"
$inWorkspacePackage = $false
$version = $null

foreach ($line in Get-Content -LiteralPath $manifestPath) {
    if ($line -match '^\s*\[(.+)\]\s*$') {
        $inWorkspacePackage = $Matches[1] -eq "workspace.package"
        continue
    }

    if ($inWorkspacePackage -and $line -match '^\s*version\s*=\s*"([^"]+)"') {
        $version = $Matches[1]
        break
    }
}

if (-not $version) {
    throw "Workspace version not found in Cargo.toml"
}

Push-Location $projectRoot
try {
    cargo build -p desktop --release
    if ($LASTEXITCODE -ne 0) {
        throw "The release build failed with exit code $LASTEXITCODE"
    }

    $iscc = Get-Command iscc -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source -First 1
    if (-not $iscc) {
        $isccCandidates = @(
            (Join-Path $env:LOCALAPPDATA "Programs\Inno Setup 6\ISCC.exe"),
            (Join-Path ${env:ProgramFiles(x86)} "Inno Setup 6\ISCC.exe"),
            (Join-Path $env:ProgramFiles "Inno Setup 6\ISCC.exe")
        )
        $iscc = $isccCandidates | Where-Object { Test-Path -LiteralPath $_ } | Select-Object -First 1
    }

    if (-not $iscc) {
        throw "ISCC.exe was not found. Run: winget install -e --id JRSoftware.InnoSetup"
    }

    $outputDir = Join-Path $PSScriptRoot "output"
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null

    & $iscc "/DAppVersion=$version" (Join-Path $PSScriptRoot "amategeko.iss")
    if ($LASTEXITCODE -ne 0) {
        throw "Inno Setup failed with exit code $LASTEXITCODE"
    }

    $setupPath = Join-Path $outputDir "amategeko-yumuhanda-$version-windows-setup.exe"
    if (-not (Test-Path -LiteralPath $setupPath)) {
        throw "Expected installer was not created: $setupPath"
    }

    $setupFile = Get-Item -LiteralPath $setupPath
    $setupHash = (Get-FileHash -LiteralPath $setupPath -Algorithm SHA256).Hash
    Write-Output $setupFile.FullName
    Write-Output "$($setupFile.Length) bytes"
    Write-Output "SHA-256: $setupHash"
    Write-Output "minisign -Sm `"$($setupFile.FullName)`""

    if ($Portable) {
        $portablePath = Join-Path $outputDir "amategeko-yumuhanda-$version-windows-portable.zip"
        Compress-Archive -LiteralPath (Join-Path $projectRoot "target\release\amategeko.exe") -DestinationPath $portablePath -Force
        $portableFile = Get-Item -LiteralPath $portablePath
        $portableHash = (Get-FileHash -LiteralPath $portablePath -Algorithm SHA256).Hash
        Write-Output $portableFile.FullName
        Write-Output "$($portableFile.Length) bytes"
        Write-Output "SHA-256: $portableHash"
        Write-Output "minisign -Sm `"$($portableFile.FullName)`""
    }
}
finally {
    Pop-Location
}
