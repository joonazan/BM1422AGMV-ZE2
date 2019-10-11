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

write(0x1b, 0xc2)
write(0x5c, 0x00)
write(0x5d, 0x00)

write(0x1c, 0x0c)

def adjust_offset():
    x = 9999

    for wk_dat in range(1, 96):
        write(0x6c, wk_dat)
        write(0x1d, 0x40)

        while read(0x18) == 0:
            pass

        datax = read_component(0x10)
        print("datax", datax)

        if diff_x > abs(datax):
            best = wk_dat
            diff_x = abs(datax)

        print ("best offset", best)
        write(0x6c, best)

with open('two_sines.txt', 'wb') as f:
    while True:
        write(0x1d, 0x40)
        while read(0x18) == 0:
            pass

        x = read_component(0x10)
        y = read_component(0x12)
        z = read_component(0x14)

        f.write('%i %i %i \n' % (x, y, z))
