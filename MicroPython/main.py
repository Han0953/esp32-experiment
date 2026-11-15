"""MicroPython ESP32 DevKit V1 (38-pin): LED GPIO25 + OLED SSD1306 I2C 128x64.

Wiring:
  OLED VCC -> 3V3, GND -> GND, SDA -> GPIO21, SCL -> GPIO22
  GPIO25 -> 220-330 ohm resistor -> LED anode (+), LED cathode (-) -> GND

Single-file SSD1306 driver: no external ssd1306.py dependency.
"""

from machine import Pin, I2C
from time import sleep_ms
import framebuf

LED_PIN = 25
SDA_PIN = 21
SCL_PIN = 22
OLED_ADDR = 0x3C
OLED_WIDTH = 128
OLED_HEIGHT = 64


class SSD1306:
    def __init__(self, i2c, addr=OLED_ADDR):
        self.i2c = i2c
        self.addr = addr
        self.buffer = bytearray(OLED_WIDTH * OLED_HEIGHT // 8)
        self.fb = framebuf.FrameBuffer(
            self.buffer, OLED_WIDTH, OLED_HEIGHT, framebuf.MONO_VLSB
        )

        # SSD1306: 128x64, internal charge pump, horizontal addressing.
        for cmd in (
            0xAE,             # display off
            0xD5, 0x80,       # display clock
            0xA8, 0x3F,       # multiplex (64 rows)
            0xD3, 0x00,       # display offset
            0x40,             # start line 0
            0x8D, 0x14,       # charge pump on
            0x20, 0x00,       # horizontal memory addressing
            0xA1,             # segment remap
            0xC8,             # COM scan direction
            0xDA, 0x12,       # COM pins for 128x64
            0x81, 0xCF,       # contrast
            0xD9, 0xF1,       # precharge
            0xDB, 0x40,       # VCOM detect
            0xA4,             # use RAM content
            0xA6,             # normal display
            0x2E,             # disable scrolling
            0xAF,             # display on
        ):
            self.command(cmd)
        self.fill(0)
        self.show()

    def command(self, cmd):
        self.i2c.writeto(self.addr, bytes((0x00, cmd)))

    def fill(self, color):
        self.fb.fill(color)

    def text(self, message, x, y):
        self.fb.text(message, x, y, 1)

    def hline(self, x, y, width):
        self.fb.hline(x, y, width, 1)

    def show(self):
        self.command(0x21)  # column address range
        self.command(0)
        self.command(OLED_WIDTH - 1)
        self.command(0x22)  # page address range
        self.command(0)
        self.command((OLED_HEIGHT // 8) - 1)

        # Chunked I2C writes for compatibility with different ESP32 builds.
        for offset in range(0, len(self.buffer), 32):
            self.i2c.writeto(self.addr, b"\x40" + self.buffer[offset:offset + 32])


led = Pin(LED_PIN, Pin.OUT, value=0)
i2c = I2C(0, sda=Pin(SDA_PIN), scl=Pin(SCL_PIN), freq=400000)

addresses = i2c.scan()
print("I2C devices:", [hex(addr) for addr in addresses])

oled = None
if OLED_ADDR in addresses:
    try:
        oled = SSD1306(i2c)
        print("OLED SSD1306 detected at", hex(OLED_ADDR))
    except OSError as err:
        print("OLED initialization failed:", err)
else:
    print("OLED 0x3C not found; LED test will still run.")

counter = 0
while True:
    state = counter % 2 == 0
    led.value(1 if state else 0)
    status_str = "LED : NYALA [5s]" if state else "LED : MATI [5s]"

    if oled is not None:
        try:
            oled.fill(0)
            oled.text("ESP32 + PYTHON", 0, 0)
            oled.hline(0, 11, OLED_WIDTH)
            oled.text(status_str, 0, 15)
            oled.text("PIN : GPIO25 (OUT)", 0, 27)
            oled.text("OLED: I2C (SDA21/SCL22)", 0, 39)
            oled.text("CHIP: DEVKIT V1 38P", 0, 51)
            oled.show()
        except OSError as err:
            print("OLED I2C error:", err)
            oled = None

    print("Status:", "LED NYALA" if state else "LED MATI", "| Menunggu 5 detik... (count:", counter, ")")
    counter += 1
    sleep_ms(5000)

