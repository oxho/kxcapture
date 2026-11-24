# List every file in the Windows test installers and fail when an OCR-off
# package still contains the worker or OCR assets. Other "snow" paths are
# printed and are not a failure.
[CmdletBinding()]
param(
    [string]$BuildDirectory = 'build/snow-shot-msvc-release'
)

$ErrorActionPreference = 'Stop'
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '../..')).Path
$build = Join-Path $repoRoot $BuildDirectory
Add-Type -AssemblyName System.IO.Compression.FileSystem

function Get-SevenZip {
    foreach ($name in @('7z', '7z.exe')) {
        $command = Get-Command $name -ErrorAction SilentlyContinue
        if ($command) { return $command.Source }
    }
    foreach ($candidate in @(
            "$env:ProgramFiles\7-Zip\7z.exe",
            ${env:ProgramFiles(x86)} + '\7-Zip\7z.exe')) {
        if ($candidate -and (Test-Path -LiteralPath $candidate)) { return $candidate }
    }
    return $null
}

function Get-SevenZipPaths([string]$Seven, [string]$Path) {
    $listing = & $Seven l -slt -ba $Path
    if ($LASTEXITCODE -ne 0) { throw "7z could not list $Path" }
    return @($listing | Where-Object { $_ -like 'Path = *' } | ForEach-Object { $_.Substring(7) } | Where-Object { $_ })
}

function Get-ArchivePaths([string]$Path) {
    $seven = Get-SevenZip
    if (-not $seven) {
        if ($Path.EndsWith('.zip', [StringComparison]::OrdinalIgnoreCase)) {
            $archive = [IO.Compression.ZipFile]::OpenRead($Path)
            try { return @($archive.Entries | ForEach-Object { $_.FullName }) }
            finally { $archive.Dispose() }
        }
        throw "7z is required to list the NSIS installer $Path"
    }
    $paths = @(Get-SevenZipPaths $seven $Path)
    $nested = @($paths | Where-Object { $_ -match '(?i)\.7z$' })
    if ($nested.Count -eq 0) { return $paths }
    $temp = Join-Path ([IO.Path]::GetTempPath()) ("kxcapture-nsis-" + [guid]::NewGuid().ToString('N'))
    New-Item -ItemType Directory -Path $temp | Out-Null
    try {
        & $seven x "-o$temp" -y $Path | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "7z could not extract $Path" }
        $expanded = [System.Collections.Generic.List[string]]::new()
        foreach ($entry in $paths) { $expanded.Add($entry) }
        foreach ($entry in $nested) {
            $inner = Join-Path $temp ($entry.Replace('/', '\'))
            if (-not (Test-Path -LiteralPath $inner -PathType Leaf)) { continue }
            foreach ($innerPath in (Get-SevenZipPaths $seven $inner)) {
                $expanded.Add("$entry!$innerPath")
            }
        }
        return @($expanded)
    }
    finally {
        Remove-Item -LiteralPath $temp -Recurse -Force
    }
}

function Test-OcrPayloadPath([string]$Path) {
    $normalized = $Path.Replace('\', '/')
    return ($normalized -match '(?i)kxcapture-ocr' -or
        $normalized -match '(?i)snow-ocr-process' -or
        $normalized -match '(?i)(^|/)assets/ocr(/|$)' -or
        $normalized -match '(?i)\.onnx$')
}

$patterns = @(
    'kxcapture-*-windows-*-setup.exe',
    'kxcapture-*-windows-*-offline.exe',
    'kxcapture-*-windows-*-portable.zip',
    'kxcapture-mini-*-windows-*-setup.exe',
    'kxcapture-mini-*-windows-*-portable.zip'
)
$packages = foreach ($pattern in $patterns) {
    Get-ChildItem -LiteralPath $build -File -Filter $pattern -ErrorAction SilentlyContinue
}
$packages = @($packages | Sort-Object Name -Unique)
if ($packages.Count -lt 5) {
    throw "Expected the setup, offline, portable, Mini setup, and Mini portable packages in $build"
}

$failed = $false
foreach ($package in $packages) {
    Write-Output ""
    Write-Output "===== $($package.Name) ====="
    $paths = @(Get-ArchivePaths $package.FullName | Where-Object { $_ -and ($_ -notmatch '/$') })
    foreach ($entry in ($paths | Sort-Object)) { Write-Output $entry }
    $ocr = @($paths | Where-Object { Test-OcrPayloadPath $_ })
    if ($ocr.Count -gt 0) {
        Write-Output "FATAL: $($package.Name) contains OCR payload files:"
        $ocr | ForEach-Object { Write-Output "FATAL: $_" }
        $failed = $true
    }
    $snow = @($paths | Where-Object { $_ -match '(?i)snow' -and -not (Test-OcrPayloadPath $_) })
    if ($snow.Count -gt 0) {
        Write-Output "INFO: other paths containing 'snow' in $($package.Name):"
        $snow | ForEach-Object { Write-Output "INFO: $_" }
    }
}
if ($failed) { throw 'OCR-off installer contents include the OCR worker or OCR assets.' }
Write-Output 'OCR-off installer gate passed.'
