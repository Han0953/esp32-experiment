# ESP32 Experiment (Rust, MicroPython, C++)

Eksperimen dan perbandingan implementasi firmware **ESP32** menggunakan **Rust (`no_std`)** sebagai codebase utama, disertai **MicroPython** dan **C++ (Arduino)** sebagai pembanding.

## Target Hardware & Wiring

* **Board**: ESP32 DevKit V1 (38-pin / 30-pin, USB-to-Serial CP2102)
* **OLED SSD1306 I2C (128x64, alamat `0x3C`)**:
  * `VCC` → `3V3`
  * `GND` → `GND`
  * `SDA` → `GPIO21`
  * `SCL` → `GPIO22`
* **LED Eksternal**:
  * `GPIO25` → Resistor `220–330Ω` → Anoda LED (`+`)
  * Katoda LED (`-`) → `GND`

## Struktur Proyek

```text
.
├── Rust/esp32-experiment/      # Codebase utama (Rust no_std + esp-hal)
│   └── src/bin/main.rs         # Dashboard OLED real-time & kontrol LED 5 detik
├── MicroPython/
│   └── main.py                 # Pembanding MicroPython (driver SSD1306 single-file)
├── C++/esp32_test/
│   └── esp32_test.ino          # Pembanding C++ (Arduino + Adafruit_SSD1306)
├── run.ps1 / run.bat           # Shortcut build, flash, dan monitor Rust
└── monitor.ps1 / monitor.bat   # Shortcut buka serial monitor tanpa flash ulang
```

## Fitur Utama (Rust)

* Berjalan *bare-metal* (`#![no_std]`, `#![no_main]`) di **240 MHz** (`CpuClock::max()`).
* **LED Blink 5 detik** (5 detik nyala, 5 detik mati) pada `GPIO25`.
* **Dashboard OLED Real-time** (refresh tiap 1 detik):
  * Frekuensi CPU (`240 MHz [DUAL]`)
  * Sisa Heap RAM (`esp-alloc`)
  * Uptime & counter siklus
  * Status LED + hitung mundur

## Cara Menjalankan (Rust)

Pastikan toolchain `esp` (`espup`, `espflash`) sudah terpasang dan ESP32 terhubung via USB:

* **Build, Flash & Serial Monitor**:
  ```powershell
  .\run
  ```
* **Buka Serial Monitor Saja** *(tanpa flash ulang)*:
  ```powershell
  .\monitor
  ```
  * `Ctrl + R`: Reset ESP32
  * `Ctrl + C`: Keluar dari serial monitor

> **Tips**: Jika proses flashing berhenti di `Connecting...`, tekan dan tahan tombol **BOOT** pada board ESP32 selama 1–2 detik hingga proses penulisan flash berjalan.
