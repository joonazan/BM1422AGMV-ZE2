import smbus, time, math

b = smbus.SMBus(1)

address = 0xf

def read (x):
    return b.read_byte_data(address, x)

def write(a, v):
    b.write_byte_data(address, a, v)

def read_component(register):
    unsigned = b.read_word_data(address, register)
    return (unsigned & 0x7fff) - (unsigned & 0x8000)

# continous 14-bit measurement at 1kHz
write(0x1b, 0b11011000)

write(0x5c, 0x00)
write(0x5d, 0x00)

# turn on DRDY pin
write(0x1c, 0x0c)

write(0x1d, 0x40)
while True:
    start_time = time.time()

    #while read(0x18) == 0:
    #    pass

    x = read_component(0x10)
    y = read_component(0x12)
    z = read_component(0x14)

    print time.time() - start_time
    print '%i %i %i \n' % (x, y, z)
