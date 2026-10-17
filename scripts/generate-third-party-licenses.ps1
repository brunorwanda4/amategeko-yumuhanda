$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$projectRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$outputPath = Join-Path $projectRoot "assets\THIRD_PARTY_LICENSES.txt"

Push-Location $projectRoot
try {
    cargo about generate --workspace --locked --offline about.hbs --output-file $outputPath
    if ($LASTEXITCODE -ne 0) {
        throw "cargo-about failed with exit code $LASTEXITCODE"
    }

    $rendered = [System.IO.File]::ReadAllText($outputPath)
    $decoded = [System.Net.WebUtility]::HtmlDecode($rendered)
    while ($decoded -ne $rendered) {
        $rendered = $decoded
        $decoded = [System.Net.WebUtility]::HtmlDecode($rendered)
    }
    $decoded = (($decoded -split "\r?\n") | ForEach-Object { $_.TrimEnd() }) -join [Environment]::NewLine
    $decoded = $decoded.TrimEnd() + [Environment]::NewLine
    [System.IO.File]::WriteAllText(
        $outputPath,
        $decoded,
        [System.Text.UTF8Encoding]::new($false)
    )
}
finally {
    Pop-Location
}
