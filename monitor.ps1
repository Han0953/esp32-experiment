# Script untuk langsung buka Serial Monitor ESP32 tanpa re-flash
$cargoBin = "$env:USERPROFILE\.cargo\bin"
if ($env:Path -notlike "*$cargoBin*") {
    $env:Path = "$cargoBin;" + $env:Path
}

Write-Host "Membuka Serial Monitor ESP32 (Tekan Ctrl+C untuk keluar)..." -ForegroundColor Cyan
espflash monitor
