#include <Arduino.h>
#include <WiFi.h>
#include <WiFiClient.h>
#include <MQTT.h>

/* === WIFI / MQTT === */

const char BROKER[] = "192.168.1.200";
const uint16_t BROKER_PORT = 1883;

const char ssid[] = "CPE-Domotique";
const char password[] = "astro4student";

static MQTTClient client;
static WiFiClient net;

/* === NOTES === */

/* CAN BE USEFULL

int B3 = 246;
int C4 = 261;
int Db4 = 277;
int D4 = 293;
int Eb4 = 311;
int E4 = 329;
int F4 = 349;
int Gb4 = 370;
int G4 = 392;
int Ab4 = 415;
int A_4 = 440;
int Bb4 = 466;
int B4 = 493;
*/

/* === PINS === */

int pwmChannel = 0;
int resolution = 14;
int audio_output_pin = 26;

/* === MUSIC === */

int bpm = 140;
float one_measure = 60.0/bpm*1000000;
float one_beat = one_measure / 4;

/* === FUNCTIONS === */

void connect() {
	// Checking WiFi
    Serial.println("checking wifi...");
    while (WiFi.status() != WL_CONNECTED) {
        Serial.print(".");
        delay(1000);
    }
    Serial.println("\nconnecting...");
	
	// Checking MQTT
    while (!client.connect("arduino-julo-2", "public", "public")) {
        Serial.print(".");
        delay(1000);
    }

	// Subscribe to the /speaker topic
    client.subscribe("/speaker");
	
    Serial.println("\nconnected!");
}

void on_mqtt(String &topic, String &payload) {
    Serial.println("incoming: " + topic + " - " + payload);

    if (topic == "/speaker") {
        ledcSetup(pwmChannel, payload.toInt(), resolution);
    }
}

/* === SETUP === */

void setup(){
    // put your setup code here, to run once:
    Serial.begin(115200);

    Serial.println("Simple clients with wifi ");
    if (strlen(ssid)==0)
        Serial.println("****** PLEASE MODIFY ssid/password *************");

	// Connect to WiFi
    WiFi.begin(ssid, password);
	
	// Connect to the MQTT Broker
    client.begin(BROKER, BROKER_PORT, net);
    client.onMessage(on_mqtt);


    // Attach audio output
    ledcSetup(pwmChannel, 1, resolution);
    ledcAttachPin(audio_output_pin, pwmChannel);
    ledcWrite(pwmChannel, pow(2, resolution)/2); // 1.65 V (c'est le prof d'electronique qui m'a donné cette formule)
}

/* === LOOP === */

void loop(){
    client.loop();

    if (not client.connected()) {
        connect();
		
        Serial.println("Connected to " + String(ssid) + "\nIP address: " + WiFi.localIP());
		
        return;
    }

	// LE CODE CI-DESSOUS EST JUSTE PRESENT POUR NE PAS L'OUBLIER (RIP)


	// SIMPLE MELODY
    /*
	
    ledcSetup(pwmChannel, E4, resolution);
    delay(1000);

    ledcSetup(pwmChannel, G4, resolution);
    delay(500);

    ledcSetup(pwmChannel, E4, resolution);
    delay(500);

    ledcSetup(pwmChannel, D4, resolution);
    delay(500);

    ledcSetup(pwmChannel, C4, resolution);
    delay(1000);

    ledcSetup(pwmChannel, B3, resolution);
    delay(2000);
    */


	// SIMPLE KICK (with audio gradient)
    /*
    for (int i = 440; i > 82; i=i-10) {
        ledcSetup(pwmChannel, i, resolution);
        delayMicroseconds(one_beat/2/4/(440-82)*10);
    }

    for (int i = 82; i > 55; i=i-10) {
        //Serial.println(i);
        ledcSetup(pwmChannel, i, resolution);
        delayMicroseconds(one_beat/2/4*3/(82-55)*10);
    }

    delayMicroseconds(one_beat*2+one_beat/2);

    for (int i = 55; i > 10; i=i-10) {
        //Serial.println(i);
        ledcSetup(pwmChannel, i, resolution);
        delayMicroseconds(one_beat/(55-10)*10);
    }*/


	// KICK + SNARE + HI-HAT (c'est loin d'être parfait)
    /*

    for (int i = 440; i > 82; i=i-10) {
        ledcSetup(pwmChannel, i, resolution);
        delayMicroseconds(one_beat/2/4/(440-82)*10);
    }

    for (int i = 82; i > 55; i=i-10) {
        ledcSetup(pwmChannel, i, resolution);
        delayMicroseconds(one_beat/2/4*3/(82-55)*10);
    }
    delayMicroseconds(one_beat/2);

    // ---

    ledcSetup(pwmChannel, 220, resolution);
    delayMicroseconds(one_beat/4*2);
    ledcSetup(pwmChannel, 1, resolution);
    delayMicroseconds(one_beat/4*2);

    // ---

    ledcSetup(pwmChannel, 196, resolution);
    delayMicroseconds(one_beat);
    ledcSetup(pwmChannel, 1, resolution);

    delayMicroseconds(one_beat);

    // ---

    ledcSetup(pwmChannel, 220, resolution);
    delayMicroseconds(one_beat/4*2);
    ledcSetup(pwmChannel, 1, resolution);
    delayMicroseconds(one_beat/4*2);

    */
}