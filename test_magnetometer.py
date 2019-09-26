import smbus

b = smbus.SMBus(1)

address = 0xf

def read (x):
    return b.read_byte_data(address, x)

def write(a, v):
    b.write_byte_data(address, a, v)

write(0x1b, 0xc2)
write(0x5c, 0x00)
write(0x5d, 0x00)

write(0x1c, 0x0c)

diff_x = 9999

for wk_dat in range(0, 96):
    write(0x6c, wk_dat)
    write(0x1d, 0x40)

    while read(0x18) == 0:
        pass

    datax = b.read_word_data(address, 0x10)
    datax = (datax & 0x7fff) - (datax & 0x8000)
    print("datax", datax)

    if diff_x > abs(datax):
        best = wk_dat
        diff_x = abs(datax)

print ("best offset", best)

write(0x6c, best)
