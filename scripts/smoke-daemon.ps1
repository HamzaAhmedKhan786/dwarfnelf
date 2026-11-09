$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$daemon = Join-Path $root 'target\debug\dwarfnelfd.exe'

if (-not (Test-Path $daemon)) {
    cargo build --manifest-path (Join-Path $root 'Cargo.toml') --bin dwarfnelfd
}

$proc = Start-Process -FilePath $daemon -PassThru -WindowStyle Hidden
try {
    $healthy = $false
    for ($i = 0; $i -lt 30; $i++) {
        try { $response = Invoke-RestMethod 'http://127.0.0.1:8765/health'; $healthy = $response.status -eq 'ok'; if ($healthy) { break } } catch { Start-Sleep -Milliseconds 200 }
    }
    if (-not $healthy) { throw 'daemon did not become healthy' }
    $caps = Invoke-RestMethod 'http://127.0.0.1:8765/v1/capabilities'
    if ($caps.api_version -ne 'v1') { throw 'unexpected API version' }
    $body = @{ max_ram_gb = 4; max_vram_gb = 0; max_ssd_gb = 20; cpu_target_percent = 50 } | ConvertTo-Json
    $budget = Invoke-RestMethod 'http://127.0.0.1:8765/v1/budgets' -Method Put -ContentType 'application/json' -Body $body
    if ($budget.cpu_target_percent -ne 50) { throw 'budget update failed' }
    Write-Output 'daemon smoke test passed'
} finally {
    if ($proc -and -not $proc.HasExited) { Stop-Process -Id $proc.Id -Force }
}
