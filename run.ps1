# Script untuk build & flash ESP32 Rust langsung dari root folder D:\ESP32 Experiment
param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$CargoArgs
)

# Pastikan path toolchain ESP32 & Cargo selalu termuat
$espPaths = @(
    "$env:USERPROFILE\.rustup\toolchains\esp\xtensa-esp-elf\bin",
    "$env:USERPROFILE\.rustup\toolchains\esp\xtensa-esp32-elf-clang\esp-clang\bin",
    "$env:USERPROFILE\.cargo\bin"
)

foreach ($p in $espPaths) {
    if (Test-Path $p) {
        if ($env:Path -notlike "*$p*") {
            $env:Path = "$p;" + $env:Path
        }
    }
}

$projectDir = Join-Path $PSScriptRoot "Rust\esp32-experiment"

if (-not (Test-Path $projectDir)) {
    Write-Error "Folder project tidak ditemukan di $projectDir"
    exit 1
}

Write-Host "Menjalankan ESP32 Rust Project ($projectDir)..." -ForegroundColor Cyan

Push-Location $projectDir
try {
    cargo run @CargoArgs
}
finally {
    Pop-Location
}
