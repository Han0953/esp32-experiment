@echo off
setlocal
set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
echo [INFO] Membuka Serial Monitor ESP32 (Tekan Ctrl+C untuk keluar)...
espflash monitor %*
