import asyncio
import sys
from time import sleep
from bleak import BleakClient
import paho.mqtt.client as mqtt

def on_connect(client, userdata, flags, rc):
    print("Connected with result code "+str(rc))

async def main(address):
  async with BleakClient(address) as client:
    if (not client.is_connected):
      raise "client not connected"

    services = await client.get_services()

    for service in services:
      print('\nservice', service.handle, service.uuid, service.description)
      print(service)
      characteristics = service.characteristics

      for char in characteristics:
        print('  characteristic', char.handle, char.uuid, char.description, char.properties)
        
        descriptors = char.descriptors

        for desc in descriptors:
          print('    descriptor', desc)


    while True:
        data = await client.read_gatt_char(char.uuid)
        cnt = int.from_bytes(data, "little")

        print("\nPrint value of last characteristic : {val}".format(val=cnt))
        
        if cnt > 2000:
            val_to_send = int(cnt / 4095 * 128)
        else:
            val_to_send = 0
            
        print(f"Sending to MQTT: {val_to_send}")
        mqtt_client.publish("/air_flow", str(val_to_send))
        
        sleep(1);


if __name__ == "__main__":
  print("Starting MQTT")
  mqtt_client = mqtt.Client()
  mqtt_client.connect("192.168.1.200", 1883, 60)
  mqtt_client.on_connect = on_connect
  print("MQTT Started")
  
  print("Starting bluetooth")
  address = "F0:08:D1:C8:80:0A"
  print('address:', address)
  asyncio.run(main(address))