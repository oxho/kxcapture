# Copy the installers and portable archives produced by scripts/package-snow-shot.ps1
# and print the version resources plus SHA-256 digests. No Authenticode signing.
[CmdletBinding()]
param(
    [string]$BuildDirectory = 'build/snow-shot-msvc-release',
    [string]$StageDirectory = 'artifacts/snow-shot',
    [string]$OutputDirectory = 'artifacts/windows'
)

$ErrorActionPreference = 'Stop'
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '../..')).Path
$build = Join-Path $repoRoot $BuildDirectory
$stage = Join-Path $repoRoot $StageDirectory
$out = Join-Path $repoRoot $OutputDirectory
if (Test-Path -LiteralPath $out) { Remove-Item -LiteralPath $out -Recurse -Force }
New-Item -ItemType Directory -Path $out | Out-Null

$patterns = @(
    'kxcapture-*-windows-*-setup.exe',
    'kxcapture-*-windows-*-offline.exe',
    'kxcapture-*-windows-*-portable.zip',
    'kxcapture-mini-*-windows-*-setup.exe',
    'kxcapture-mini-*-windows-*-portable.zip'
)
$files = foreach ($pattern in $patterns) {
    Get-ChildItem -LiteralPath $build -File -Filter $pattern -ErrorAction SilentlyContinue
}
$files = @($files | Sort-Object Name -Unique)
if ($files.Count -eq 0) { throw "No installer or portable archive was found in $build" }
foreach ($file in $files) {
    Copy-Item -LiteralPath $file.FullName -Destination (Join-Path $out $file.Name)
}

function Show-VersionResource([string]$Path) {
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        Write-Output "Version resource missing: $Path"
        return
    }
    $info = (Get-Item -LiteralPath $Path).VersionInfo
    Write-Output "VersionInfo path=$Path"
    Write-Output "ProductName=$($info.ProductName)"
    Write-Output "FileDescription=$($info.FileDescription)"
    Write-Output "FileVersion=$($info.FileVersion)"
    Write-Output "ProductVersion=$($info.ProductVersion)"
    Write-Output "OriginalFilename=$($info.OriginalFilename)"
    Write-Output "CompanyName=$($info.CompanyName)"
}

Show-VersionResource (Join-Path $stage 'bin/kxcapture.exe')
Show-VersionResource (Join-Path $repoRoot 'artifacts/snow-shot-mini/bin/kxcapture_mini.exe')
Get-ChildItem -LiteralPath $out -File | ForEach-Object { Show-VersionResource $_.FullName }

$sums = foreach ($file in (Get-ChildItem -LiteralPath $out -File | Sort-Object Name)) {
    $hash = (Get-FileHash -LiteralPath $file.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
    Write-Output ("size_bytes {0} {1}" -f $file.Length, $file.Name)
    '{0}  {1}' -f $hash, $file.Name
}
$sums | Set-Content -LiteralPath (Join-Path $out 'SHA256SUMS.txt') -Encoding ascii
Get-Content -LiteralPath (Join-Path $out 'SHA256SUMS.txt')
