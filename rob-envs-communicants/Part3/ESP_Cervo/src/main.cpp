#include <Arduino.h>
#include <WiFi.h>
#include <WiFiClient.h>
#include <ESP32Servo.h>
#include <MQTT.h>
#include <ArduinoQueue.h>

Servo servo;

const char BROKER[] = "192.168.1.200";
const uint16_t BROKER_PORT = 1883;

const char ssid[] = "CPE-Domotique";
const char password[] = "astro4student";

static MQTTClient client;
static WiFiClient net;

const int SERVO_PIN = 13;
static ArduinoQueue<int> angle_queue;
static int last_angle = 0;

void connect() {
  Serial.println("checking wifi...");
  while (WiFi.status() != WL_CONNECTED) {
    Serial.print(".");
    delay(1000);
  }

  Serial.println("\nconnecting...");
  while (!client.connect("arduino-johan", "public", "public")) {
    Serial.println(".");
    delay(1000);
  }

  client.subscribe("/air_flow");
  Serial.println("\nconnected!");
}

void on_mqtt(String &topic, String &payload) {  
  if (topic == "/air_flow") {
    int angle = payload.toInt();
    angle_queue.enqueue(angle);
  } else {
    Serial.println("incoming: " + topic + " - " + payload);
  }
  // Note: Do not use the client in the callback to publish, subscribe or
  // unsubscribe as it may cause deadlocks when other things arrive while
  // sending and receiving acknowledgments. Instead, change a global variable,
  // or push to a queue and handle it in the loop after calling `client.loop()`.
}

void setup() {
  // put your setup code here, to run once:
  Serial.begin(115200);
  servo.attach(SERVO_PIN, 520, 2100);

	Serial.println("Simple clients with wifi ");
	if (strlen(ssid)==0)
		Serial.println("****** PLEASE MODIFY ssid/password *************");

  WiFi.begin(ssid, password);
	client.begin(BROKER, BROKER_PORT, net);
  client.onMessage(on_mqtt);

  // Reset servo
  servo.write(last_angle);
  delay(100);
}

void loop() {
  client.loop();

  if (not client.connected()) {
    connect();
    Serial.println("Connected to " + String(ssid) + "\nIP address: " + WiFi.localIP());
    return;
  }

  if (not angle_queue.isEmpty()) {
    int angle = angle_queue.dequeue();
    if (angle != last_angle) {
      Serial.println("Set angle: " + String(angle));
      servo.write(angle);
      last_angle = angle;
      delay(500);
    } else {
      Serial.println("Not set angle: " + String(angle));
    }
  }
}
