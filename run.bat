@echo off
setlocal

rem Path toolchain
set "PATH=%USERPROFILE%\.rustup\toolchains\esp\xtensa-esp-elf\bin;%USERPROFILE%\.rustup\toolchains\esp\xtensa-esp32-elf-clang\esp-clang\bin;%USERPROFILE%\.cargo\bin;%PATH%"

set "PROJECT_DIR=%~dp0Rust\esp32-experiment"

if not exist "%PROJECT_DIR%" (
    echo [ERROR] Project folder tidak ditemukan di %PROJECT_DIR%
    exit /b 1
)

echo [INFO] Menjalankan ESP32 Rust Project (%PROJECT_DIR%)...
pushd "%PROJECT_DIR%"
cargo run %*
popd
