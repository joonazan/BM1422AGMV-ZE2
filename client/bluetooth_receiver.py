from bluepy.btle import Peripheral, UUID, DefaultDelegate
import struct

class MyDelegate(DefaultDelegate):
    def __init__(self):
        DefaultDelegate.__init__(self)

    def handleNotification(self, handle, data):
        if handle == readings.valHandle:
            print(*struct.unpack('ffff', data), flush=True)

p = Peripheral("3C:71:BF:CB:1E:42")
p.setDelegate(MyDelegate())

readings = next(c for c in p.getCharacteristics() if c.uuid == UUID("59462f12-9543-9999-12c8-58b459a2712d"))

# enable notification
p.writeCharacteristic(readings.valHandle + 1, (1).to_bytes(2, byteorder='little'))

while p.waitForNotifications(1.0):
    pass

print("Error, a second passed with no notification received")
