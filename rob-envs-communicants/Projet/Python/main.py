import paho.mqtt.client as mqtt
from time import sleep
import keyboard
from bleak import BleakClient
import asyncio

# Global variables (ugly Python)
live = False
bpm = 100
delay = 1
delays = [1, 2, 4, 8, 16]
recorded_notes = []

# Translates received input to real key with its frequency
input_to_key = {
    1: [261, 'C'],
    2: [277, 'C#'],
    3: [293, 'D'],
    4: [311, 'D#'],
    5: [329, 'E'],
    6: [349, 'F'],
    7: [370, 'F#'],
    8: [392, 'G'],
    9: [415, 'G#'],
    10: [440, 'A'],
    11: [466, 'A#'],
    12: [493, 'B'],
    13: [1, '_']
}

# Keyboard key to real key with its frequency
keyboard_to_key = {
    'a': [261, 'C'],
    'z': [277, 'C#'],
    'e': [293, 'D'],
    'r': [311, 'D#'],
    'q': [329, 'E'],
    's': [349, 'F'],
    'd': [370, 'F#'],
    'f': [392, 'G'],
    'w': [415, 'G#'],
    'x': [440, 'A'],
    'c': [466, 'A#'],
    'v': [493, 'B'],
    'space': [1, '_']
}

# Send a note to the speakers over MQTT
def send_note(note):
    print(f"sending {note[0]}Hz")
    client.publish("/speaker", str(note[0]))
    sleep(60/bpm/note[1])

# Process a pad input
def process_input(input):
    # Ugly Python part
    global live
    global bpm
    global delay
    global delays
    global recorded_notes
    
    # Process notes
    if input in input_to_key:
        key = input_to_key[input]
        
        if live:
            send_note([key[0], delay])
        else:
            recorded_notes.append([key[0], delay])
            
        print(f"{delay}/4 - {len(recorded_notes)+1} - {key[1]}")
    
    # LIVE
    if input == 14:
        live = not live
        print("Live", "ON" if live else "OFF")
        client.publish("/speaker", str(1))
    
    # DELETE SEQUENCE
    if not live and input == 15:
        print('Delete sequence')
        print()
        client.publish("/speaker", str(1))
        recorded_notes = []
    
    # PLAY SEQUENCE
    if not live and input == 16:
        print()
        print('Playing sequence')
        for note in recorded_notes:
            send_note(note)
        client.publish("/speaker", str(1))
        print()
      
    # ADD BPM
    if input == 17 and bpm < 400:
        bpm = bpm + 10
        print(f"bpm: {bpm}")
     
    # SUBSTRACT BPM
    if input == 18 and bpm > 10:
        bpm = bpm - 10
        print(f"bpm: {bpm}")
    
    # MULTIPLY MEASURE
    if input == 19 and delay > 1:
        delay = delays[delays.index(delay) - 1]
        print(f"delay: {delay}/4")
    
    # DIVIDE MEASURE
    if input == 20 and delay < 16:
        delay = delays[delays.index(delay) + 1]
        print(f"delay: {delay}/4")
        

# The callback for when the client receives a CONNACK response from the server.
def on_connect(client, userdata, flags, rc):
    print("Connected with result code " + str(rc))
    # Subscribing in on_connect() means that if we lose the connection and
    # reconnect then subscriptions will be renewed.

async def main(address):
    global recorded_notes
    global keyboard_to_key
    global delay
    
    async with BleakClient(address) as ble_client:
        if (not ble_client.is_connected):
            raise "client not connected"

        print("Connected")
        services = await ble_client.get_services()
    
        # Bluetooth services
        for service in services:
            print('\nservice', service.handle, service.uuid, service.description)
            print(service)
            characteristics = service.characteristics

            # Characteristics
            for char in characteristics:
                print('  characteristic', char.handle, char.uuid, char.description, char.properties)
        
                descriptors = char.descriptors

                # Characteristics descriptions
                for desc in descriptors:
                    print('    descriptor', desc)
                   
        # Main loop
        while True:
            data = await ble_client.read_gatt_char(char.uuid)
            cnt = int.from_bytes(data, "little")

            #print(f"Print value of last characteristic : {cnt}")
            process_input(cnt)
            
            
            # ==== KEYBOARD ====
            
            # Je n'ai pas eu le temps de combiner pad + clavier, mais ça aurait été l'histoire d'une heure pas plus
            
            """
            keyboard_key = keyboard.read_key()
        
            if keyboard_key in keyboard_to_key:
                key = keyboard_to_key[keyboard_key]
                recorded_notes.append([key[0], delay])
                print(f"{delay}/4 - {len(recorded_notes)+1} - {key[1]}")
            
            
            if keyboard.is_pressed('enter'):
                print()
                print('Playing sequence')
                for note in recorded_notes:
                    print(f"sending {note[0]}Hz")
                    client.publish("/speaker", str(note[0]))
                    sleep(60/bpm/note[1])
                client.publish("/speaker", str(1))
                print()
                    
            if keyboard.is_pressed('backspace'):
                print('Delete sequence')
                print()
                client.publish("/speaker", str(1))
                recorded_notes = []
                
            if keyboard.is_pressed('up arrow') and bpm < 400:
                bpm = bpm + 10
                print(f"bpm: {bpm}")
                
            if keyboard.is_pressed('down arrow') and bpm > 10:
                bpm = bpm - 10
                print(f"bpm: {bpm}")
                
            if keyboard.is_pressed('left arrow') and delay > 1:
                delay = delays[delays.index(delay) - 1]
                print(f"delay: {delay}/4")
                
            if keyboard.is_pressed('right arrow') and delay < 16:
                delay = delays[delays.index(delay) + 1]
                print(f"delay: {delay}/4")
            """

if __name__ == "__main__":
    bpm = 100
    delay = 1
    delays = [1, 2, 4, 8, 16]
    note = 1
    recorded_notes = []
    
    print("Connecting to MQTT...")
    client = mqtt.Client()
    client.on_connect = on_connect

    client.connect("192.168.1.200", 1883, 60)
    print("Connencted")
    
    
    print("Starting bluetooth")
    address = "34:85:18:01:7F:66"
    print('address:', address)
    asyncio.run(main(address))
