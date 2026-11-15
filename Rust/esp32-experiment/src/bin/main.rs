#![no_std]
#![no_main]

// Board: ESP32 DevKit V1, 38 pin, CP2102 (ESP32 klasik)
// OLED : SSD1306 I2C 128x64, VCC=3V3, SDA=GPIO21, SCL=GPIO22
// LED  : GPIO25 -> resistor 220-330 ohm -> anoda LED, katoda -> GND

use core::fmt::Write;
use core::panic::PanicInfo;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::{Baseline, Text},
};
use esp_hal::clock::CpuClock;
use esp_hal::{
    gpio::{Level, Output, OutputConfig},
    i2c::master::{Config as I2cConfig, I2c},
    time::{Duration, Instant, Rate},
};
use esp_println::println;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

// Alamat SSD1306 umumnya 0x3C; beberapa modul menggunakan 0x3D.
const OLED_ADDR: u8 = 0x3C;
const BLINK_MS: u64 = 5000;

// Descriptor aplikasi dibutuhkan bootloader ESP-IDF standar.
esp_bootloader_esp_idf::esp_app_desc!();

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC: {info}");
    loop {
        core::hint::spin_loop();
    }
}

fn wait_ms(ms: u64) {
    let start = Instant::now();
    while start.elapsed() < Duration::from_millis(ms) {
        core::hint::spin_loop();
    }
}

#[esp_hal::main]
fn main() -> ! {
    esp_alloc::heap_allocator!(size: 64 * 1024);

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let free_heap = esp_alloc::HEAP.free();
    let used_heap = esp_alloc::HEAP.used();
    println!("ESP32 Rust + LED + OLED mulai! Free heap: {free_heap}, Used: {used_heap}");

    // LED eksternal aktif-HIGH.
    let mut led = Output::new(peripherals.GPIO25, Level::Low, OutputConfig::default());

    // I2C 100 kHz agar stabil pada kabel jumper biasa.
    let i2c = I2c::new(
        peripherals.I2C0,
        I2cConfig::default().with_frequency(Rate::from_khz(100)),
    )
    .expect("Konfigurasi I2C tidak valid")
    .with_sda(peripherals.GPIO21)
    .with_scl(peripherals.GPIO22);

    let interface = I2CDisplayInterface::new_custom_address(i2c, OLED_ADDR);
    let mut oled = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    if let Err(err) = oled.init() {
        println!("OLED gagal init di alamat {OLED_ADDR:#04X}: {err:?}");
        println!("LED tetap berkedip. Cek wiring, alamat I2C, dan controller OLED.");
        loop {
            led.toggle();
            wait_ms(BLINK_MS);
        }
    }

    println!("OLED siap di {OLED_ADDR:#04X} | SDA21 SCL22 | LED GPIO25");
    let text = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

    let boot_time = Instant::now();
    let mut counter: u32 = 0;

    loop {
        let elapsed_secs = boot_time.elapsed().as_secs();
        let led_on = (elapsed_secs / 5) % 2 == 0;
        let sisa_sec = 5 - (elapsed_secs % 5);
        led.set_level(if led_on { Level::High } else { Level::Low });

        let free_bytes = esp_alloc::HEAP.free();
        let free_kb = free_bytes / 1024;
        let mins = elapsed_secs / 60;
        let secs = elapsed_secs % 60;

        let mut ram_buf = heapless::String::<32>::new();
        let _ = write!(ram_buf, "RAM : {} KB / 64KB", free_kb);

        let mut time_buf = heapless::String::<32>::new();
        let _ = write!(time_buf, "UP  : {:02}:{:02} | #{}", mins, secs, counter);

        let mut led_buf = heapless::String::<32>::new();
        let status_str = if led_on { "NYALA" } else { "MATI " };
        let _ = write!(led_buf, "LED : [{}] sisa {}s", status_str, sisa_sec);

        println!(
            "[TELEMETRI #{counter}] Up: {mins:02}:{secs:02} | CPU: 240MHz | Free Heap: {free_bytes}B | LED: {status_str}"
        );

        // Render ulang framebuffer lalu kirim ke OLED (128x64).
        let _ = oled.clear(BinaryColor::Off);

        // Header
        let _ = Text::with_baseline("ESP32 LIVE DASHBOARD", Point::new(0, 0), text, Baseline::Top)
            .draw(&mut oled);
        let _ = Line::new(Point::new(0, 11), Point::new(127, 11))
            .into_styled(line_style)
            .draw(&mut oled);

        // Baris 1: CPU Clock
        let _ = Text::with_baseline("CPU : 240 MHz [DUAL]", Point::new(0, 15), text, Baseline::Top)
            .draw(&mut oled);

        // Baris 2: RAM
        let _ = Text::with_baseline(&ram_buf, Point::new(0, 27), text, Baseline::Top)
            .draw(&mut oled);

        // Baris 3: Uptime & Counter
        let _ = Text::with_baseline(&time_buf, Point::new(0, 39), text, Baseline::Top)
            .draw(&mut oled);

        // Baris 4: Status LED + Countdown
        let _ = Text::with_baseline(&led_buf, Point::new(0, 51), text, Baseline::Top)
            .draw(&mut oled);

        let _ = oled.flush();

        counter += 1;
        wait_ms(1000); // Update setiap 1 detik secara real-time!
    }
}