$processName = "serenya"
Write-Host "Started monitoring $processName (Updates every 5 seconds)..."

while ($true) {
    $proc = Get-Process -Name $processName -ErrorAction SilentlyContinue
    if ($proc) {
        $ramMB = [math]::Round($proc.WorkingSet64 / 1MB, 2)
        $cpuCounter = Get-Counter "\Process($processName)\% Processor Time" -ErrorAction SilentlyContinue
        $cpuPercent = 0
        if ($cpuCounter) {
            $cpuPercent = [math]::Round($cpuCounter.CounterSamples[0].CookedValue, 2)
        }
        $time = Get-Date -Format "HH:mm:ss"
        Write-Host "[$time] RAM: ${ramMB} MB | CPU: ${cpuPercent}%"
    } else {
        $time = Get-Date -Format "HH:mm:ss"
        Write-Host "[$time] Process '$processName' is not running."
    }
    Start-Sleep -Seconds 5
}
