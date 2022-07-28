//Server Based Tempature Monitor

#include <Adafruit_Sensor.h>
#include <DHT.h>
#include <DHT_U.h>

#include <WiFi.h>

#define DHTPIN 2      /* pin number */
#define DHTTYPE    DHT11 

const char* ssid     = "your-ssid";
const char* password = "your-password";

const char* host = "your url";
const int hostport = 0000;

DHT_Unified dht(DHTPIN, DHTTYPE);

void setup() {
  Serial.begin(9600);

  //Wifi Setup
  WiFi.begin(ssid, password);

  while (WiFi.status() != WL_CONNECTED) {
      delay(500);
      Serial.print(".");
  }

  // Initialize device.
  dht.begin();
}

void loop() {
// Delay between measurements.
  delay(5000);
  // Get temperature event and print its value.
  sensors_event_t event;
  dht.temperature().getEvent(&event);
  if (isnan(event.temperature)) {
    Serial.println(F("Error reading temperature!"));
  }
  else {
    Serial.print(F("Temperature: "));
    Serial.print(event.temperature);
    send_string_to_server(event.temperature);
    Serial.println(F("°C"));
  }
  // Get humidity event and print its value.
  dht.humidity().getEvent(&event);
  if (isnan(event.relative_humidity)) {
    Serial.println(F("Error reading humidity!"));
  }
  else {
    Serial.print(F("Humidity: "));
    Serial.print(event.relative_humidity);
    Serial.println(F("%"));
  }
}


//This is the function that sends the data to the server.
void send_string_to_server(float data) {
  WiFiClient client;


  if(WiFi.status() == WL_CONNECTED){
    if(!client.connect(host, hostport)){
      Serial.println("Error connecting to server!");
      return;
    }

    client.print(String(data));
    client.stop();
  }
}