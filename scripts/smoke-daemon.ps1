$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$manifest = Join-Path $root 'Cargo.toml'
$metadata = cargo metadata --manifest-path $manifest --no-deps --format-version 1 --locked
if ($LASTEXITCODE -ne 0) { throw 'cargo metadata failed' }
$target = ($metadata | ConvertFrom-Json).target_directory
cargo build --manifest-path $manifest --locked --bin dwarfnelfd
if ($LASTEXITCODE -ne 0) { throw 'Daemon build failed' }

$daemon = Join-Path $target 'debug/dwarfnelfd.exe'
$scratch = Join-Path ([IO.Path]::GetTempPath()) "dwarfnelf-smoke-$([guid]::NewGuid())"
New-Item -ItemType Directory -Path $scratch | Out-Null
$saved = @{}
foreach ($name in @('DWARFNELF_TOKEN', 'DWARFNELF_DB', 'DWARFNELF_PORT')) {
    $saved[$name] = [Environment]::GetEnvironmentVariable($name, 'Process')
}
$proc = $null
try {
    $env:DWARFNELF_TOKEN = [Convert]::ToHexString([Security.Cryptography.RandomNumberGenerator]::GetBytes(32))
    $env:DWARFNELF_DB = Join-Path $scratch 'state.sqlite'
    $env:DWARFNELF_PORT = '0'
    $log = Join-Path $scratch 'daemon.log'
    $proc = Start-Process -FilePath $daemon -PassThru -WindowStyle Hidden -RedirectStandardOutput $log
    $base = $null
    for ($i = 0; $i -lt 50; $i++) {
        if ($proc.HasExited) { throw "Daemon exited with code $($proc.ExitCode)" }
        $line = Get-Content -LiteralPath $log | Select-Object -First 1
        if ($line) {
            $address = ($line | ConvertFrom-Json).fields.addr
            if ($address -match '^127\.0\.0\.1:[0-9]+$') {
                $base = "http://$address"
                break
            }
        }
        Start-Sleep -Milliseconds 100
    }
    if (-not $base) { throw 'Daemon did not report its bound loopback port' }
    $headers = @{ Authorization = "Bearer $env:DWARFNELF_TOKEN" }
    $health = Invoke-RestMethod "$base/health" -TimeoutSec 5
    if ($health.status -ne 'ok') { throw 'Health check failed' }
    $unauthenticated = Invoke-WebRequest "$base/v1/budgets" -SkipHttpErrorCheck -TimeoutSec 5
    if ($unauthenticated.StatusCode -ne 401) { throw 'Unauthenticated request was not rejected' }
    $caps = Invoke-RestMethod "$base/v1/capabilities" -Headers $headers -TimeoutSec 5
    if ($caps.features -notcontains 'knowledge_lexical') { throw 'Lexical retrieval capability is missing' }
    $hardware = Invoke-RestMethod "$base/v1/hardware" -Headers $headers -TimeoutSec 5
    if ($hardware.logical_cpus -lt 1) { throw 'CPU discovery failed' }
    $body = @{ max_ram_gb = 4; max_ssd_gb = 20; cpu_target_percent = 50 } | ConvertTo-Json
    Invoke-RestMethod "$base/v1/budgets" -Method Put -Headers $headers -ContentType 'application/json' -Body $body -TimeoutSec 5 | Out-Null
    $budget = Invoke-RestMethod "$base/v1/budgets" -Headers $headers -TimeoutSec 5
    if ($budget.cpu_target_percent -ne 50) { throw 'Budget readback failed' }
    $document = @{ source = 'smoke-fixture'; text = 'SQLite supports local lexical retrieval.' } | ConvertTo-Json
    Invoke-RestMethod "$base/v1/knowledge/import" -Method Post -Headers $headers -ContentType 'application/json' -Body $document -TimeoutSec 5 | Out-Null
    $result = Invoke-RestMethod "$base/v1/knowledge/search" -Method Post -Headers $headers -ContentType 'application/json' -Body '{"query":"SQLite"}' -TimeoutSec 5
    if ($result.abstained -or $result.hits[0].source -ne 'smoke-fixture') { throw 'Lexical citation check failed' }
    $admission = Invoke-RestMethod "$base/v1/decisions/admission" -Method Post -Headers $headers -ContentType 'application/json' -Body '{"weights_bytes":5000000000,"kv_cache_bytes":1,"runtime_bytes":1}' -TimeoutSec 5
    if ($admission.outcome -ne 'exceeds_configured_budget') { throw 'CPU admission check failed' }
    $workflow = Invoke-RestMethod "$base/v1/agents/runs" -Method Post -Headers $headers -ContentType 'application/json' -Body '{"steps":[{"tool":"knowledge_search","query":"SQLite"}]}' -TimeoutSec 5
    if ($workflow.status -ne 'completed' -or $workflow.citations[0].source -ne 'smoke-fixture') { throw 'Agent workflow check failed' }
    $history = Invoke-RestMethod "$base/v1/agents/runs/$($workflow.id)" -Headers $headers -TimeoutSec 5
    if ($history.summary.trace[0].hit_count -ne 1 -or $null -ne $history.summary.citations) { throw 'Agent summary check failed' }
    Write-Output 'Daemon smoke test passed'
} finally {
    if ($proc -and -not $proc.HasExited) {
        Stop-Process -Id $proc.Id
        if (-not $proc.WaitForExit(5000)) { throw 'Smoke daemon did not exit' }
    }
    foreach ($name in $saved.Keys) { [Environment]::SetEnvironmentVariable($name, $saved[$name], 'Process') }
    foreach ($name in @('daemon.log', 'state.sqlite', 'state.sqlite-wal', 'state.sqlite-shm')) {
        $artifact = Join-Path $scratch $name
        if (Test-Path -LiteralPath $artifact -PathType Leaf) { Remove-Item -LiteralPath $artifact }
    }
    [IO.Directory]::Delete($scratch)
}
