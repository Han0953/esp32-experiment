#include <Adafruit_GFX.h>
#include <Adafruit_SSD1306.h>
#include <Wire.h>

#define LED_PIN 25
#define SDA_PIN 21
#define SCL_PIN 22
#define OLED_ADDR 0x3C

#define SCREEN_WIDTH 128
#define SCREEN_HEIGHT 64

Adafruit_SSD1306 oled(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, -1);

bool ledState = false;
bool oledReady = false;
unsigned long lastBlink = 0;
unsigned long counter = 0;

void updateOLED() {
  if (!oledReady)
    return;

  oled.clearDisplay();
  oled.setTextSize(1);
  oled.setTextColor(SSD1306_WHITE);

  oled.setCursor(0, 0);
  oled.println("ESP32 + C++");

  oled.drawLine(0, 11, 127, 11, SSD1306_WHITE);

  oled.setCursor(0, 15);
  oled.println(ledState ? "LED : NYALA [5s]" : "LED : MATI [5s]");

  oled.setCursor(0, 27);
  oled.println("PIN : GPIO25 (OUT)");

  oled.setCursor(0, 39);
  oled.println("OLED: I2C (SDA21/SCL22)");

  oled.setCursor(0, 51);
  oled.println("CHIP: DEVKIT V1 38P");

  oled.display();
}

void setup() {
  Serial.begin(115200);

  pinMode(LED_PIN, OUTPUT);
  digitalWrite(LED_PIN, LOW);

  Wire.begin(SDA_PIN, SCL_PIN);

  Wire.beginTransmission(OLED_ADDR);

  if (Wire.endTransmission() == 0) {
    oledReady = oled.begin(SSD1306_SWITCHCAPVCC, OLED_ADDR);
  }

  Serial.println("ESP32 C++ Started");
  updateOLED();
  lastBlink = millis();
}

void loop() {
  if (millis() - lastBlink >= 5000) {
    lastBlink = millis();

    ledState = !ledState;
    digitalWrite(LED_PIN, ledState);

    counter++;

    Serial.printf(
        "Status: %s | Menunggu 5 detik... (Count: %lu)\n",
        ledState ? "LED NYALA" : "LED MATI", counter
    );

    updateOLED();
  }
}

