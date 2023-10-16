#include <Arduino.h>
#include <WiFi.h>
#include <WiFiClient.h>
#include <MQTT.h>

/* ==== WIFI & MQTT ==== */

const char BROKER[] = "192.168.1.178";
const uint16_t BROKER_PORT = 1883;

const char ssid[] = "CPE-Domotique";
const char password[] = "astro4student";

WiFiClient net;
MQTTClient client;

unsigned long lastMillis = 0;

/* ==== GPIO ==== */

const int TEMP_PIN = 36;

void connect() {
    Serial.println("checking wifi...");
    while (WiFi.status() != WL_CONNECTED) {
        Serial.print(".");
        delay(1000);
    }

    Serial.println("\nconnecting...");
    while (!client.connect("arduino-julien", "public", "public")) {
        Serial.print(".");
        delay(1000);
    }

    Serial.println();
    Serial.println("\nconnected!");
}

void setup()
{
    Serial.begin(115200);
    delay(500);

    Serial.println("Simple clients with wifi");
    if (strlen(ssid)==0) {
        Serial.println("****** PLEASE MODIFY ssid/password *************");
    }

    WiFi.mode(WIFI_STA);
    WiFi.begin(ssid, password);

    while (WiFi.status() != WL_CONNECTED) {
        delay(500);
        Serial.print('.');
    }

    Serial.println();
    Serial.println("Connected to " + String(ssid) + "IP address: " + WiFi.localIP());

    client.begin(BROKER, BROKER_PORT, net);
    Serial.println("Connected to Broker");
}

void loop()
{
    client.loop();

    if (!client.connected()) {
        connect();
    }

    // publish a message roughly every second.
    if (millis() - lastMillis > 1000) {
        lastMillis = millis();

        int sensorValue = analogRead(TEMP_PIN);
        auto stringedSensor = String(sensorValue);
        Serial.print("Read : ");
        Serial.println(sensorValue);

        client.publish("/temperature", stringedSensor);
    }
}