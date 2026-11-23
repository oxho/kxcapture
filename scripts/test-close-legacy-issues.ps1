$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

function Assert([bool]$Condition, [string]$Message) {
    if (-not $Condition) { throw $Message }
}

$testDirectory = Join-Path ([IO.Path]::GetTempPath()) "kxcapture-legacy-issues-test-$([guid]::NewGuid().ToString('N'))"
$null = New-Item -ItemType Directory -Path $testDirectory
$mockGh = Join-Path $testDirectory 'gh-mock.ps1'
$logFile = Join-Path $testDirectory 'calls.jsonl'
@'
param([Parameter(ValueFromRemainingArguments)][string[]]$GhArguments)
Add-Content -LiteralPath $env:SNOW_TEST_GH_LOG -Value (($GhArguments -join ' '))
throw "GitHub CLI must not be called: $($GhArguments -join ' ')"
'@ | Set-Content -LiteralPath $mockGh -Encoding utf8
$env:SNOW_TEST_GH_LOG = $logFile
$script = Join-Path $PSScriptRoot 'close-legacy-issues.ps1'

foreach ($arguments in @(@{}, @{ Apply = $true; MutationDelaySeconds = 0 })) {
    $failed = $false
    $message = ''
    try {
        & $script -GhExecutable $mockGh @arguments | Out-Null
    } catch {
        $failed = $true
        $message = $_.Exception.Message
    }
    Assert $failed 'The historical issue script must refuse to run.'
    Assert ($message -match 'historical and must not be executed') "Unexpected refusal: $message"
}
Assert (-not (Test-Path -LiteralPath $logFile)) 'The historical issue script called GitHub.'
Write-Output 'close-legacy-issues.ps1 refuses to run and does not call GitHub.'
