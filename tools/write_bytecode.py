"""
TODO(pbz): Take out all the loops. Then scrape out special symbol and
TODO(pbz): compare it against the actual assembly output
! It can be indirect but not have immediate data :(
! DO VALIDATION SO THAT IF DIRECT + INDEX FOR OP1, THAT IS AN ERROR!
!                                           v <- SHOULD HAVE @ HERE
! RIGHT NOW, IT JUST LOOKS LIKE THIS: MOVqw  R1(-3, -3), @R2(-3, -3)
"""

import random
from itertools import cycle
from natural_indexing import encode

RANDOM_SIGNED_NUMBERS_X16 = [random.randint(-32768, 32767) for i in range(100)]
random_signed = cycle(iter(RANDOM_SIGNED_NUMBERS_X16))

RANDOM_SIGNED_NUMBERS_20 = [random.randint(-20, 20) for i in range(100)]
random_signed_20 = cycle(iter(i for i in RANDOM_SIGNED_NUMBERS_20 if i))

# TODO(pbz): Allow creation of any bit length natural indices
NATIND16 = 36879
NATIND32 = 2954019116
NATIND64 = 11529215048034579760

natural_indices = {
    'w': (NATIND16, 2),
    'd': (NATIND32, 4),
    'q': (NATIND64, 8),
}

def pad(width, string):
    return '0' * (width - len(string)) + string

def lower(bc, op, bit8=False, bit7=False, byte1=None, arg1=None, arg2=None):
    if bit8:
        op = op | (1 << 7)  # Set 8th bit

    if bit7:
        op = op | (1 << 6)  # Set 7th bit

    bc.write(op.to_bytes(1, 'big'))

    if byte1:
        bc.write(byte1.to_bytes(1, 'big'))


def arg(bc, value, width, endianness, signed=False):
    bc.write(value.to_bytes(width, endianness, signed=signed))


def write_bytecode():
    with open('bc-example.bin', 'wb') as bc:
        # bc.write(0b00000100.to_bytes(1, 'big'))  # RET

        # bc.write(0b00101010_00000001.to_bytes(2, 'big'))  # STORESP R1, FLAGS
        # bc.write(0b00101010_00010001.to_bytes(2, 'big'))  # STORESP R1, IP

        # bc.write(0b00101001_00010000.to_bytes(2, 'big'))  # LOADSP FLAGS, R1
        # bc.write(0b00101001_00010001.to_bytes(2, 'big'))  # LOADSP IP, R1

        # bc.write(0b00000000_00000011.to_bytes(2, 'big'))  # BREAK 3

        # bc.write(0b00000010.to_bytes(1, 'big'))  # JMP8 -3
        # bc.write((-3).to_bytes(1, 'big', signed=True))  # ..

        # bc.write(0b01000010.to_bytes(1, 'big'))  # JMP8 -3
        # bc.write((-3).to_bytes(1, 'big', signed=True))  # ..

        # bc.write(0b10000010.to_bytes(1, 'big'))  # JMP8cc -3
        # bc.write((-3).to_bytes(1, 'big', signed=True))  # ..

        # bc.write(0b11000010.to_bytes(1, 'big'))  # JMP8cs -3
        # bc.write((-3).to_bytes(1, 'big', signed=True))  # ..

        # bc.write(0b00110110_00000001.to_bytes(2, 'big'))  # POPn R1
        # bc.write(0b00110101_00000001.to_bytes(2, 'big'))  # PUSHn R1
        # bc.write(0b00110110_00001001.to_bytes(2, 'big'))  # POPn @R1
        # bc.write(0b00110101_00001001.to_bytes(2, 'big'))  # PUSHn @R1

        # bc.write(0b10110110_00000001.to_bytes(2, 'big'))  # POPn R1 -3
        # bc.write((-3).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b10110101_00000001.to_bytes(2, 'big'))  # PUSHn R1 -3
        # bc.write((-3).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b10110110_00001001.to_bytes(2, 'big'))  # POPn @R1(-3, -3)
        # bc.write((36879).to_bytes(2, 'little'))  # ..

        # bc.write(0b10110101_00001001.to_bytes(2, 'big'))  # PUSHn @R1(-3, -3)
        # bc.write((36879).to_bytes(2, 'little'))  # ..

        # # 32
        # bc.write(0b00101100_00000001.to_bytes(2, 'big'))  # POP32 R1
        # bc.write(0b00101011_00000001.to_bytes(2, 'big'))  # PUSH32 R1
        # bc.write(0b00101100_00001001.to_bytes(2, 'big'))  # POP32 @R1
        # bc.write(0b00101011_00001001.to_bytes(2, 'big'))  # PUSH32 @R1

        # bc.write(0b10101100_00000001.to_bytes(2, 'big'))  # POP32 R1 -3
        # bc.write((-3).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b10101011_00000001.to_bytes(2, 'big'))  # PUSH32 R1 -3
        # bc.write((-3).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b10101100_00001001.to_bytes(2, 'big'))  # POP32 @R1(-3, -3)
        # bc.write((36879).to_bytes(2, 'little'))  # ..

        # bc.write(0b10101011_00001001.to_bytes(2, 'big'))  # PUSH32 @R1(-3, -3)
        # bc.write((36879).to_bytes(2, 'little'))  # ..

        # # 64
        # bc.write(0b01101100_00000001.to_bytes(2, 'big'))  # POP64 R1
        # bc.write(0b01101011_00000001.to_bytes(2, 'big'))  # PUSH64 R1
        # bc.write(0b01101100_00001001.to_bytes(2, 'big'))  # POP64 @R1
        # bc.write(0b01101011_00001001.to_bytes(2, 'big'))  # PUSH64 @R1

        # bc.write(0b11101100_00000001.to_bytes(2, 'big'))  # POP64 R1 -3
        # bc.write((-3).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b11101011_00000001.to_bytes(2, 'big'))  # PUSH64 R1 -3
        # bc.write((-3).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b11101100_00001001.to_bytes(2, 'big'))  # POP64 @R1(-3, -3)
        # bc.write((36879).to_bytes(2, 'little'))  # ..

        # bc.write(0b11101011_00001001.to_bytes(2, 'big'))  # PUSH64 @R1(-3, -3)
        # bc.write((36879).to_bytes(2, 'little'))  # ..

        # # 32
        # bc.write(0b00000011_00010001.to_bytes(2, 'big'))  # CALL32 R1
        # bc.write(0b00000011_00000001.to_bytes(2, 'big'))  # CALL32a R1

        # bc.write(0b10000011_00010001.to_bytes(2, 'big'))  # CALL32 R1 -3
        # bc.write((-3).to_bytes(4, 'little', signed=True))  # ..

        # bc.write(0b10000011_00000001.to_bytes(2, 'big'))  # CALL32a R1 -3
        # bc.write((-3).to_bytes(4, 'little', signed=True))  # ..

        # lower(bc, 0x03, True, False, 0b00011001)  # CALL32 @R1(-300, -300)
        # arg(bc, 2954019116, 4, 'little')  # ..

        # lower(bc, 0x03, False, False, 0b00011001)  # CALL32 @R1

        # # CALL32a @R1(-300, -300)
        # bc.write(0b10000011_00001001.to_bytes(2, 'big'))
        # bc.write((2954019116).to_bytes(4, 'little'))  # ..

        # bc.write(0b00000011_00110001.to_bytes(2, 'big'))  # CALL32EX R1

        # bc.write(0b00000011_00100001.to_bytes(2, 'big'))  # CALL32EXa R1

        # lower(bc, 0x03, False, False, 0b00111001)  # CALL32EX @R1

        # lower(bc, 0x03, False, False, 0b00101001)  # CALL32EXa @R1

        # bc.write(0b10000011_00110001.to_bytes(2, 'big'))  # CALL32EX R1 -3
        # bc.write((-3).to_bytes(4, 'little', signed=True))  # ..

        # bc.write(0b10000011_00100001.to_bytes(2, 'big'))  # CALL32EXa R1 -3
        # bc.write((-3).to_bytes(4, 'little', signed=True))  # ..

        # # CALL32EX @R1(-300, -300)
        # bc.write(0b10000011_00111001.to_bytes(2, 'big'))
        # bc.write((2954019116).to_bytes(4, 'little'))  # ..

        # # CALL32EXa @R1(-300, -300)
        # bc.write(0b10000011_00101001.to_bytes(2, 'big'))
        # bc.write((2954019116).to_bytes(4, 'little'))  # ..

        # # 64
        # bc.write(0b11000011_00110001.to_bytes(2, 'big'))  # CALL64EX -3
        # bc.write((-3).to_bytes(8, 'little', signed=True))  # ..

        # bc.write(0b11000011_00100001.to_bytes(2, 'big'))  # CALL64EXa -3
        # bc.write((-3).to_bytes(8, 'little', signed=True))  # ..

        # # 32
        # bc.write(0b00000001_00000001.to_bytes(2, 'big'))  # JMP32 R1
        # bc.write(0b00000001_10000001.to_bytes(2, 'big'))  # JMP32cc R1
        # bc.write(0b00000001_11000001.to_bytes(2, 'big'))  # JMP32cs R1

        # bc.write(0b00000001_00010001.to_bytes(2, 'big'))  # JMP32 R1  ;; Rel
        # bc.write(0b00000001_10010001.to_bytes(2, 'big'))  # JMP32cc R1  ;; Rel
        # bc.write(0b00000001_11010001.to_bytes(2, 'big'))  # JMP32cs R1  ;; Rel

        # bc.write(0b10000001_00000001.to_bytes(2, 'big'))  # JMP32 R1 -3
        # bc.write((-3).to_bytes(4, 'little', signed=True))  # ..

        # bc.write(0b10000001_00010001.to_bytes(2, 'big'))  # JMP32 R1 -3  ;; Rel
        # bc.write((-3).to_bytes(4, 'little', signed=True))  # ..

        # # JMP32 @R1(-300, -300)
        # bc.write(0b10000001_00001001.to_bytes(2, 'big'))
        # bc.write((2954019116).to_bytes(4, 'little'))  # ..

        # # JMP32 @R1(-300, -300)  ;; Rel
        # bc.write(0b10000001_00011001.to_bytes(2, 'big'))
        # bc.write((2954019116).to_bytes(4, 'little'))  # ..

        # bc.write(0b00000001_00011001.to_bytes(2, 'big'))  # JMP32 @R1  ;; Rel

        # bc.write(0b00000001_00001001.to_bytes(2, 'big'))  # JMP32 @R1  ;; Abs

        # # 64
        # bc.write(0b01000001_00000001.to_bytes(2, 'big'))  # JMP64 1000
        # bc.write((1000).to_bytes(8, 'little'))  # ..

        # bc.write(0b01000001_00010001.to_bytes(2, 'big'))  # JMP64 1000  ;; Rel
        # bc.write((1000).to_bytes(8, 'little'))  # ..

        # MOVI
        # w
        # bc.write(0b01110111_00000001.to_bytes(2, 'big'))  # MOVIbw R1 1000
        # bc.write((1000).to_bytes(2, 'little'))  # ..

        # bc.write(0b01110111_00010001.to_bytes(2, 'big'))  # MOVIww R1 1000
        # bc.write((1000).to_bytes(2, 'little'))  # ..

        # bc.write(0b01110111_00100001.to_bytes(2, 'big'))  # MOVIdw R1 1000
        # bc.write((1000).to_bytes(2, 'little'))  # ..

        # bc.write(0b01110111_00110001.to_bytes(2, 'big'))  # MOVIqw R1 1000
        # bc.write((1000).to_bytes(2, 'little'))  # ..

        # # d
        # bc.write(0b10110111_00000001.to_bytes(2, 'big'))  # MOVIbd R1 1000
        # bc.write((1000).to_bytes(4, 'little'))  # ..

        # bc.write(0b10110111_00010001.to_bytes(2, 'big'))  # MOVIwd R1 1000
        # bc.write((1000).to_bytes(4, 'little'))  # ..

        # bc.write(0b10110111_00100001.to_bytes(2, 'big'))  # MOVIdd R1 1000
        # bc.write((1000).to_bytes(4, 'little'))  # ..

        # bc.write(0b10110111_00110001.to_bytes(2, 'big'))  # MOVIqd R1 1000
        # bc.write((1000).to_bytes(4, 'little'))  # ..

        # # q
        # bc.write(0b11110111_00000001.to_bytes(2, 'big'))  # MOVIbd R1 1000
        # bc.write((1000).to_bytes(8, 'little'))  # ..

        # bc.write(0b11110111_00010001.to_bytes(2, 'big'))  # MOVIwd R1 1000
        # bc.write((1000).to_bytes(8, 'little'))  # ..

        # bc.write(0b11110111_00100001.to_bytes(2, 'big'))  # MOVIdd R1 1000
        # bc.write((1000).to_bytes(8, 'little'))  # ..

        # bc.write(0b11110111_00110001.to_bytes(2, 'big'))  # MOVIqd R1 1000
        # bc.write((1000).to_bytes(8, 'little'))  # ..

        # # CMPI
        # # w
        # bc.write(0b00101101_00000001.to_bytes(2, 'big'))  # CMPI32weq R1 -1000
        # bc.write((-1000).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b01101101_00000001.to_bytes(2, 'big'))  # CMPI64weq R1 -1000
        # bc.write((-1000).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b00101110_00000001.to_bytes(2, 'big'))  # CMPI32wlte R1 -1000
        # bc.write((-1000).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b01101110_00000001.to_bytes(2, 'big'))  # CMPI64wlte R1 -1000
        # bc.write((-1000).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b00101111_00000001.to_bytes(2, 'big'))  # CMPI32wgte R1 -1000
        # bc.write((-1000).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b01101111_00000001.to_bytes(2, 'big'))  # CMPI64wgte R1 -1000
        # bc.write((-1000).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b00110000_00000001.to_bytes(2, 'big'))  # CMPI32wulte R1 1000
        # bc.write((1000).to_bytes(2, 'little'))  # ..

        # bc.write(0b01110000_00000001.to_bytes(2, 'big'))  # CMPI64wulte R1 1000
        # bc.write((1000).to_bytes(2, 'little'))  # ..

        # bc.write(0b00110001_00000001.to_bytes(2, 'big'))  # CMPI32wugte R1 1000
        # bc.write((1000).to_bytes(2, 'little'))  # ..

        # bc.write(0b01110001_00000001.to_bytes(2, 'big'))  # CMPI64wugte R1 1000
        # bc.write((1000).to_bytes(2, 'little'))  # ..

        # # d
        # bc.write(0b10101101_00000001.to_bytes(2, 'big'))  # CMPI32deq R1 -1000
        # bc.write((-1000).to_bytes(4, 'little', signed=True))  # ..

        # bc.write(0b11101101_00000001.to_bytes(2, 'big'))  # CMPI64deq R1 -1000
        # bc.write((-1000).to_bytes(4, 'little', signed=True))  # ..

        # bc.write(0b10101110_00000001.to_bytes(2, 'big'))  # CMPI32dlte R1 -1000
        # bc.write((-1000).to_bytes(4, 'little', signed=True))  # ..

        # bc.write(0b11101110_00000001.to_bytes(2, 'big'))  # CMPI64dlte R1 -1000
        # bc.write((-1000).to_bytes(4, 'little', signed=True))  # ..

        # bc.write(0b10101111_00000001.to_bytes(2, 'big'))  # CMPI32dgte R1 -1000
        # bc.write((-1000).to_bytes(4, 'little', signed=True))  # ..

        # bc.write(0b11101111_00000001.to_bytes(2, 'big'))  # CMPI64dgte R1 -1000
        # bc.write((-1000).to_bytes(4, 'little', signed=True))  # ..

        # bc.write(0b10110000_00000001.to_bytes(2, 'big'))  # CMPI32dulte R1 1000
        # bc.write((1000).to_bytes(4, 'little'))  # ..

        # bc.write(0b11110000_00000001.to_bytes(2, 'big'))  # CMPI64dulte R1 1000
        # bc.write((1000).to_bytes(4, 'little'))  # ..

        # bc.write(0b10110001_00000001.to_bytes(2, 'big'))  # CMPI32dugte R1 1000
        # bc.write((1000).to_bytes(4, 'little'))  # ..

        # bc.write(0b11110001_00000001.to_bytes(2, 'big'))  # CMPI64dugte R1 1000
        # bc.write((1000).to_bytes(4, 'little'))  # ..

        # # w @
        # bc.write(0b00101101_00011001.to_bytes(2, 'big'))  # CMPI32weq @R1(-3, -3) -1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((-1000).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b01101101_00011001.to_bytes(2, 'big'))  # CMPI64weq @R1(-3, -3) -1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((-1000).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b00101110_00011001.to_bytes(2, 'big'))  # CMPI32wlte @R1(-3, -3) -1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((-1000).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b01101110_00011001.to_bytes(2, 'big'))  # CMPI64wlte @R1(-3, -3) -1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((-1000).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b00101111_00011001.to_bytes(2, 'big'))  # CMPI32wgte @R1(-3, -3) -1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((-1000).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b01101111_00011001.to_bytes(2, 'big'))  # CMPI64wgte @R1(-3, -3) -1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((-1000).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b00110000_00011001.to_bytes(2, 'big'))  # CMPI32wulte @R1(-3, -3) 1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((1000).to_bytes(2, 'little'))  # ..

        # bc.write(0b01110000_00011001.to_bytes(2, 'big'))  # CMPI64wulte @R1(-3, -3) 1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((1000).to_bytes(2, 'little'))  # ..

        # bc.write(0b00110001_00011001.to_bytes(2, 'big'))  # CMPI32wugte @R1(-3, -3) 1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((1000).to_bytes(2, 'little'))  # ..

        # bc.write(0b01110001_00011001.to_bytes(2, 'big'))  # CMPI64wugte @R1(-3, -3) 1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((1000).to_bytes(2, 'little'))  # ..

        # # d @
        # bc.write(0b10101101_00011001.to_bytes(2, 'big'))  # CMPI32deq @R1(-3, -3) -1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((-1000).to_bytes(4, 'little', signed=True))  # ..

        # bc.write(0b11101101_00011001.to_bytes(2, 'big'))  # CMPI64deq @R1(-3, -3) -1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((-1000).to_bytes(4, 'little', signed=True))  # ..

        # bc.write(0b10101110_00011001.to_bytes(2, 'big'))  # CMPI32dlte @R1(-3, -3) -1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((-1000).to_bytes(4, 'little', signed=True))  # ..

        # bc.write(0b11101110_00011001.to_bytes(2, 'big'))  # CMPI64dlte @R1(-3, -3) -1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((-1000).to_bytes(4, 'little', signed=True))  # ..

        # bc.write(0b10101111_00011001.to_bytes(2, 'big'))  # CMPI32dgte @R1(-3, -3) -1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((-1000).to_bytes(4, 'little', signed=True))  # ..

        # bc.write(0b11101111_00011001.to_bytes(2, 'big'))  # CMPI64dgte @R1(-3, -3) -1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((-1000).to_bytes(4, 'little', signed=True))  # ..

        # bc.write(0b10110000_00011001.to_bytes(2, 'big'))  # CMPI32dulte @R1(-3, -3) 1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((1000).to_bytes(4, 'little'))  # ..

        # bc.write(0b11110000_00011001.to_bytes(2, 'big'))  # CMPI64dulte @R1(-3, -3) 1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((1000).to_bytes(4, 'little'))  # ..

        # bc.write(0b10110001_00011001.to_bytes(2, 'big'))  # CMPI32dugte @R1(-3, -3) 1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((1000).to_bytes(4, 'little'))  # ..

        # bc.write(0b11110001_00011001.to_bytes(2, 'big'))  # CMPI64dugte @R1(-3, -3) 1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((1000).to_bytes(4, 'little'))  # ..

        # # MOVIn
        # bc.write(0b01111000_00000001.to_bytes(2, 'big'))  # MOVInw R1, (-3, -3)
        # bc.write((36879).to_bytes(2, 'little'))  # ..

        # # MOVInd R1, (-300, -300)
        # bc.write(0b10111000_00000001.to_bytes(2, 'big'))
        # bc.write((2954019116).to_bytes(4, 'little'))  # ..

        # # MOVInd R1, (-30000, -30000)
        # bc.write(0b11111000_00000001.to_bytes(2, 'big'))
        # bc.write((11529215048034579760).to_bytes(8, 'little'))  # ..

        # # MOVInw @R1(-3, -3), (-3, -3)
        # bc.write(0b01111000_01001001.to_bytes(2, 'big'))
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((36879).to_bytes(2, 'little'))  # ..

        # # MOVInd @R1(-3, -3), (-300, -300)
        # bc.write(0b10111000_01001001.to_bytes(2, 'big'))
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((2954019116).to_bytes(4, 'little'))  # ..

        # # MOVInd @R1(-3, -3), (-30000, -30000)
        # bc.write(0b11111000_01001001.to_bytes(2, 'big'))
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((11529215048034579760).to_bytes(8, 'little'))  # ..

        # # MOVREL
        # bc.write(0b01111001_00000001.to_bytes(2, 'big'))  # MOVERELw R1, -1000
        # bc.write((-1000).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b10111001_00000001.to_bytes(2, 'big'))  # MOVERELd R1, -1000
        # bc.write((-1000).to_bytes(4, 'little', signed=True))  # ..

        # bc.write(0b11111001_00000001.to_bytes(2, 'big'))  # MOVERELq R1, -1000
        # bc.write((-1000).to_bytes(8, 'little', signed=True))  # ..

        # # MOVERELw @R1(-3, -3), -1000
        # bc.write(0b01111001_01001001.to_bytes(2, 'big'))
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((-1000).to_bytes(2, 'little', signed=True))  # ..

        # # MOVERELd @R1(-3, -3), -1000
        # bc.write(0b10111001_01001001.to_bytes(2, 'big'))
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((-1000).to_bytes(4, 'little', signed=True))  # ..

        # # MOVERELq @R1(-3, -3), -1000
        # bc.write(0b11111001_01001001.to_bytes(2, 'big'))
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((-1000).to_bytes(8, 'little', signed=True))  # ..

        # MOV

        # lower(bc, 0x1D, False, False, 0b00100001)  # MOVb R1, R2

        # lower(bc, 0x1E, False, False, 0b00100001)  # MOVw R1, R2

        # lower(bc, 0x1F, False, False, 0b00100001)  # MOVd R1, R2

        # lower(bc, 0x20, False, False, 0b00100001)  # MOVq R1, R2

        # lower(bc, 0x1D, False, False, 0b00101001)  # MOVb @R1, R2

        # lower(bc, 0x1E, False, False, 0b00101001)  # MOVw @R1, R2

        # lower(bc, 0x1F, False, False, 0b00101001)  # MOVd @R1, R2

        # lower(bc, 0x20, False, False, 0b00101001)  # MOVq @R1, R2

        # lower(bc, 0x1D, False, False, 0b10100001)  # MOVb R1, @R2

        # lower(bc, 0x1E, False, False, 0b10100001)  # MOVw R1, @R2

        # lower(bc, 0x1F, False, False, 0b10100001)  # MOVd R1, @R2

        # lower(bc, 0x20, False, False, 0b10100001)  # MOVq R1, @R2

        # lower(bc, 0x1D, False, False, 0b10101001)  # MOVb @R1, @R2

        # lower(bc, 0x1E, False, False, 0b10101001)  # MOVw @R1, @R2

        # lower(bc, 0x1F, False, False, 0b10101001)  # MOVd @R1, @R2

        # lower(bc, 0x20, False, False, 0b10101001)  # MOVq @R1, @R2

        # lower(bc, 0x1D, True, False, 0b00101001)  # MOVbw @R1(-3, -3), R2
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x1E, True, False, 0b00101001)  # MOVww @R1(-3, -3), R2
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x1F, True, False, 0b00101001)  # MOVdw @R1(-3, -3), R2
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x20, True, False, 0b00101001)  # MOVqw @R1(-3, -3), R2
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x1D, True, False, 0b10101001)  # MOVbw @R1(-3, -3), @R2
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x1E, True, False, 0b10101001)  # MOVww @R1(-3, -3), @R2
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x1F, True, False, 0b10101001)  # MOVdw @R1(-3, -3), @R2
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x20, True, False, 0b10101001)  # MOVqw @R1(-3, -3), @R2
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x1D, False, True, 0b10100001)  # MOVbw R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x1E, False, True, 0b10100001)  # MOVww R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x1F, False, True, 0b10100001)  # MOVdw R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x20, False, True, 0b10100001)  # MOVqw R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x1D, True, True, 0b10101001)  # MOVbw @R1(-3, -3), @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x1E, True, True, 0b10101001)  # MOVww @R1(-3, -3), @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x1F, True, True, 0b10101001)  # MOVdw @R1(-3, -3), @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x20, True, True, 0b10101001)  # MOVqw @R1(-3, -3), @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x21, True, False, 0b00101001)  # MOVbd @R1(-300, -300), R2
        # arg(bc, NATIND32, 4, 'little')

        # lower(bc, 0x22, True, False, 0b00101001)  # MOVwd @R1(-300, -300), R2
        # arg(bc, NATIND32, 4, 'little')

        # lower(bc, 0x23, True, False, 0b00101001)  # MOVdd @R1(-300, -300), R2
        # arg(bc, NATIND32, 4, 'little')

        # lower(bc, 0x24, True, False, 0b00101001)  # MOVqd @R1(-300, -300), R2
        # arg(bc, NATIND32, 4, 'little')

        # lower(bc, 0x21, True, False, 0b10101001)  # MOVbd @R1(-300, -300), @R2
        # arg(bc, NATIND32, 4, 'little')

        # lower(bc, 0x22, True, False, 0b10101001)  # MOVwd @R1(-300, -300), @R2
        # arg(bc, NATIND32, 4, 'little')

        # lower(bc, 0x23, True, False, 0b10101001)  # MOVdd @R1(-300, -300), @R2
        # arg(bc, NATIND32, 4, 'little')

        # lower(bc, 0x24, True, False, 0b10101001)  # MOVqd @R1(-300, -300), @R2
        # arg(bc, NATIND32, 4, 'little')

        # lower(bc, 0x21, False, True, 0b10100001)  # MOVbd R1, @R2(-300, -300)
        # arg(bc, NATIND32, 4, 'little')

        # lower(bc, 0x22, False, True, 0b10100001)  # MOVwd R1, @R2(-300, -300)
        # arg(bc, NATIND32, 4, 'little')

        # lower(bc, 0x23, False, True, 0b10100001)  # MOVdd R1, @R2(-300, -300)
        # arg(bc, NATIND32, 4, 'little')

        # lower(bc, 0x24, False, True, 0b10100001)  # MOVqd R1, @R2(-300, -300)
        # arg(bc, NATIND32, 4, 'little')

        # lower(bc, 0x21, True, True, 0b10101001)  # MOVbd @R1(-300, -300), @R2(-300, -300)
        # arg(bc, NATIND32, 4, 'little')
        # arg(bc, NATIND32, 4, 'little')

        # lower(bc, 0x22, True, True, 0b10101001)  # MOVwd @R1(-300, -300), @R2(-300, -300)
        # arg(bc, NATIND32, 4, 'little')
        # arg(bc, NATIND32, 4, 'little')

        # lower(bc, 0x23, True, True, 0b10101001)  # MOVdd @R1(-300, -300), @R2(-300, -300)
        # arg(bc, NATIND32, 4, 'little')
        # arg(bc, NATIND32, 4, 'little')

        # lower(bc, 0x24, True, True, 0b10101001)  # MOVqd @R1(-300, -300), @R2(-300, -300)
        # arg(bc, NATIND32, 4, 'little')
        # arg(bc, NATIND32, 4, 'little')

        # lower(bc, 0x28, True, False, 0b00101001)  # MOVqq @R1(-30000, -30000), R2
        # arg(bc, NATIND64, 8, 'little')

        # lower(bc, 0x28, True, False, 0b10101001)  # MOVqq @R1(-30000, -30000), @R2
        # arg(bc, NATIND64, 8, 'little')

        # lower(bc, 0x28, False, True, 0b10100001)  # MOVqq R1, @R2(-30000, -30000)
        # arg(bc, NATIND64, 8, 'little')

        # lower(bc, 0x28, True, True, 0b10101001)  # MOVqq @R1(-30000, -30000), @R2(-30000, -30000)
        # arg(bc, NATIND64, 8, 'little')
        # arg(bc, NATIND64, 8, 'little')

        # # MOVsn
        # bc.write(0b01100101_00100001.to_bytes(2, 'big'))  # MOVsnw R1, R2 -1000
        # bc.write((-1000).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b01100110_00100001.to_bytes(2, 'big'))  # MOVsnd R1, R2 -1000
        # bc.write((-1000).to_bytes(4, 'little', signed=True))  # ..

        # bc.write(0b01100101_00101001.to_bytes(2, 'big'))  # MOVsnw @R1, R2 -1000
        # bc.write((-1000).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b01100110_00101001.to_bytes(2, 'big'))  # MOVsnd @R1, R2 -1000
        # bc.write((-1000).to_bytes(4, 'little', signed=True))  # ..

        # bc.write(0b01100101_10101001.to_bytes(2, 'big'))  # MOVsnw @R1, @R2(-3, -3)
        # bc.write((36879).to_bytes(2, 'little'))  # ..

        # bc.write(0b01100110_10101001.to_bytes(2, 'big'))  # MOVsnd @R1, @R2(-300, -300)
        # bc.write((2954019116).to_bytes(4, 'little'))  # ..

        # bc.write(0b11100101_00101001.to_bytes(2, 'big'))  # MOVsnw @R1(-3, -3), R2 -1000
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((-1000).to_bytes(2, 'little', signed=True))  # ..

        # bc.write(0b11100110_00101001.to_bytes(2, 'big'))  # MOVsnd @R1(-300, -300), R2 -1000
        # bc.write((2954019116).to_bytes(4, 'little'))  # ..
        # bc.write((-1000).to_bytes(4, 'little', signed=True))  # ..

        # bc.write(0b11100101_10101001.to_bytes(2, 'big'))  # MOVsnw @R1(-3, -3), @R2(-3, -3)
        # bc.write((36879).to_bytes(2, 'little'))  # ..
        # bc.write((36879).to_bytes(2, 'little'))  # ..

        # bc.write(0b11100110_10101001.to_bytes(2, 'big'))  # MOVsnd @R1(-300, -300), @R2(-300, -300)
        # bc.write((2954019116).to_bytes(4, 'little'))  # ..
        # bc.write((2954019116).to_bytes(4, 'little'))  # ..

        # lower(bc, 0x0C, False, False, 0b00100001)  # ADD32 R1, R2
        # lower(bc, 0x0C, False, False, 0b00101001)  # ADD32 @R1, R2
        # lower(bc, 0x0C, False, False, 0b10100001)  # ADD32 R1, @R2
        # lower(bc, 0x0C, False, False, 0b10101001)  # ADD32 @R1, @R2
        # lower(bc, 0x0C, True, False, 0b00100001)  # ADD32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x0C, True, False, 0b10100001)  # ADD32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x0C, True, False, 0b10101001)  # ADD32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x0C, False, True, 0b00100001)  # ADD64 R1, R2
        # lower(bc, 0x0C, False, True, 0b00101001)  # ADD64 @R1, R2
        # lower(bc, 0x0C, False, True, 0b10100001)  # ADD64 R1, @R2
        # lower(bc, 0x0C, False, True, 0b10101001)  # ADD64 @R1, @R2
        # lower(bc, 0x0C, True, True, 0b00100001)  # ADD64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x0C, True, True, 0b10100001)  # ADD64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x0C, True, True, 0b10101001)  # ADD64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x14, False, False, 0b00100001)  # AND32 R1, R2
        # lower(bc, 0x14, False, False, 0b00101001)  # AND32 @R1, R2
        # lower(bc, 0x14, False, False, 0b10100001)  # AND32 R1, @R2
        # lower(bc, 0x14, False, False, 0b10101001)  # AND32 @R1, @R2
        # lower(bc, 0x14, True, False, 0b00100001)  # AND32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x14, True, False, 0b10100001)  # AND32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x14, True, False, 0b10101001)  # AND32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x14, False, True, 0b00100001)  # AND64 R1, R2
        # lower(bc, 0x14, False, True, 0b00101001)  # AND64 @R1, R2
        # lower(bc, 0x14, False, True, 0b10100001)  # AND64 R1, @R2
        # lower(bc, 0x14, False, True, 0b10101001)  # AND64 @R1, @R2
        # lower(bc, 0x14, True, True, 0b00100001)  # AND64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x14, True, True, 0b10100001)  # AND64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x14, True, True, 0b10101001)  # AND64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x19, False, False, 0b00100001)  # ASHR32 R1, R2
        # lower(bc, 0x19, False, False, 0b00101001)  # ASHR32 @R1, R2
        # lower(bc, 0x19, False, False, 0b10100001)  # ASHR32 R1, @R2
        # lower(bc, 0x19, False, False, 0b10101001)  # ASHR32 @R1, @R2
        # lower(bc, 0x19, True, False, 0b00100001)  # ASHR32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x19, True, False, 0b10100001)  # ASHR32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x19, True, False, 0b10101001)  # ASHR32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x19, False, True, 0b00100001)  # ASHR64 R1, R2
        # lower(bc, 0x19, False, True, 0b00101001)  # ASHR64 @R1, R2
        # lower(bc, 0x19, False, True, 0b10100001)  # ASHR64 R1, @R2
        # lower(bc, 0x19, False, True, 0b10101001)  # ASHR64 @R1, @R2
        # lower(bc, 0x19, True, True, 0b00100001)  # ASHR64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x19, True, True, 0b10100001)  # ASHR64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x19, True, True, 0b10101001)  # ASHR64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x10, False, False, 0b00100001)  # DIV32 R1, R2
        # lower(bc, 0x10, False, False, 0b00101001)  # DIV32 @R1, R2
        # lower(bc, 0x10, False, False, 0b10100001)  # DIV32 R1, @R2
        # lower(bc, 0x10, False, False, 0b10101001)  # DIV32 @R1, @R2
        # lower(bc, 0x10, True, False, 0b00100001)  # DIV32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x10, True, False, 0b10100001)  # DIV32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x10, True, False, 0b10101001)  # DIV32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x10, False, True, 0b00100001)  # DIV64 R1, R2
        # lower(bc, 0x10, False, True, 0b00101001)  # DIV64 @R1, R2
        # lower(bc, 0x10, False, True, 0b10100001)  # DIV64 R1, @R2
        # lower(bc, 0x10, False, True, 0b10101001)  # DIV64 @R1, @R2
        # lower(bc, 0x10, True, True, 0b00100001)  # DIV64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x10, True, True, 0b10100001)  # DIV64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x10, True, True, 0b10101001)  # DIV64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x11, False, False, 0b00100001)  # DIVU32 R1, R2
        # lower(bc, 0x11, False, False, 0b00101001)  # DIVU32 @R1, R2
        # lower(bc, 0x11, False, False, 0b10100001)  # DIVU32 R1, @R2
        # lower(bc, 0x11, False, False, 0b10101001)  # DIVU32 @R1, @R2
        # lower(bc, 0x11, True, False, 0b00100001)  # DIVU32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x11, True, False, 0b10100001)  # DIVU32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x11, True, False, 0b10101001)  # DIVU32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x11, False, True, 0b00100001)  # DIVU64 R1, R2
        # lower(bc, 0x11, False, True, 0b00101001)  # DIVU64 @R1, R2
        # lower(bc, 0x11, False, True, 0b10100001)  # DIVU64 R1, @R2
        # lower(bc, 0x11, False, True, 0b10101001)  # DIVU64 @R1, @R2
        # lower(bc, 0x11, True, True, 0b00100001)  # DIVU64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x11, True, True, 0b10100001)  # DIVU64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x11, True, True, 0b10101001)  # DIVU64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x1A, False, False, 0b00100001)  # EXTNDB32 R1, R2
        # lower(bc, 0x1A, False, False, 0b00101001)  # EXTNDB32 @R1, R2
        # lower(bc, 0x1A, False, False, 0b10100001)  # EXTNDB32 R1, @R2
        # lower(bc, 0x1A, False, False, 0b10101001)  # EXTNDB32 @R1, @R2
        # lower(bc, 0x1A, True, False, 0b00100001)  # EXTNDB32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x1A, True, False, 0b10100001)  # EXTNDB32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x1A, True, False, 0b10101001)  # EXTNDB32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x1A, False, True, 0b00100001)  # EXTNDB64 R1, R2
        # lower(bc, 0x1A, False, True, 0b00101001)  # EXTNDB64 @R1, R2
        # lower(bc, 0x1A, False, True, 0b10100001)  # EXTNDB64 R1, @R2
        # lower(bc, 0x1A, False, True, 0b10101001)  # EXTNDB64 @R1, @R2
        # lower(bc, 0x1A, True, True, 0b00100001)  # EXTNDB64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x1A, True, True, 0b10100001)  # EXTNDB64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x1A, True, True, 0b10101001)  # EXTNDB64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x1C, False, False, 0b00100001)  # EXTNDD32 R1, R2
        # lower(bc, 0x1C, False, False, 0b00101001)  # EXTNDD32 @R1, R2
        # lower(bc, 0x1C, False, False, 0b10100001)  # EXTNDD32 R1, @R2
        # lower(bc, 0x1C, False, False, 0b10101001)  # EXTNDD32 @R1, @R2
        # lower(bc, 0x1C, True, False, 0b00100001)  # EXTNDD32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x1C, True, False, 0b10100001)  # EXTNDD32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x1C, True, False, 0b10101001)  # EXTNDD32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x1C, False, True, 0b00100001)  # EXTNDD64 R1, R2
        # lower(bc, 0x1C, False, True, 0b00101001)  # EXTNDD64 @R1, R2
        # lower(bc, 0x1C, False, True, 0b10100001)  # EXTNDD64 R1, @R2
        # lower(bc, 0x1C, False, True, 0b10101001)  # EXTNDD64 @R1, @R2
        # lower(bc, 0x1C, True, True, 0b00100001)  # EXTNDD64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x1C, True, True, 0b10100001)  # EXTNDD64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x1C, True, True, 0b10101001)  # EXTNDD64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x1B, False, False, 0b00100001)  # EXTNDW32 R1, R2
        # lower(bc, 0x1B, False, False, 0b00101001)  # EXTNDW32 @R1, R2
        # lower(bc, 0x1B, False, False, 0b10100001)  # EXTNDW32 R1, @R2
        # lower(bc, 0x1B, False, False, 0b10101001)  # EXTNDW32 @R1, @R2
        # lower(bc, 0x1B, True, False, 0b00100001)  # EXTNDW32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x1B, True, False, 0b10100001)  # EXTNDW32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x1B, True, False, 0b10101001)  # EXTNDW32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x1B, False, True, 0b00100001)  # EXTNDW64 R1, R2
        # lower(bc, 0x1B, False, True, 0b00101001)  # EXTNDW64 @R1, R2
        # lower(bc, 0x1B, False, True, 0b10100001)  # EXTNDW64 R1, @R2
        # lower(bc, 0x1B, False, True, 0b10101001)  # EXTNDW64 @R1, @R2
        # lower(bc, 0x1B, True, True, 0b00100001)  # EXTNDW64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x1B, True, True, 0b10100001)  # EXTNDW64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x1B, True, True, 0b10101001)  # EXTNDW64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x12, False, False, 0b00100001)  # MOD32 R1, R2
        # lower(bc, 0x12, False, False, 0b00101001)  # MOD32 @R1, R2
        # lower(bc, 0x12, False, False, 0b10100001)  # MOD32 R1, @R2
        # lower(bc, 0x12, False, False, 0b10101001)  # MOD32 @R1, @R2
        # lower(bc, 0x12, True, False, 0b00100001)  # MOD32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x12, True, False, 0b10100001)  # MOD32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x12, True, False, 0b10101001)  # MOD32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x12, False, True, 0b00100001)  # MOD64 R1, R2
        # lower(bc, 0x12, False, True, 0b00101001)  # MOD64 @R1, R2
        # lower(bc, 0x12, False, True, 0b10100001)  # MOD64 R1, @R2
        # lower(bc, 0x12, False, True, 0b10101001)  # MOD64 @R1, @R2
        # lower(bc, 0x12, True, True, 0b00100001)  # MOD64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x12, True, True, 0b10100001)  # MOD64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x12, True, True, 0b10101001)  # MOD64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x13, False, False, 0b00100001)  # MODU32 R1, R2
        # lower(bc, 0x13, False, False, 0b00101001)  # MODU32 @R1, R2
        # lower(bc, 0x13, False, False, 0b10100001)  # MODU32 R1, @R2
        # lower(bc, 0x13, False, False, 0b10101001)  # MODU32 @R1, @R2
        # lower(bc, 0x13, True, False, 0b00100001)  # MODU32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x13, True, False, 0b10100001)  # MODU32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x13, True, False, 0b10101001)  # MODU32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x13, False, True, 0b00100001)  # MODU64 R1, R2
        # lower(bc, 0x13, False, True, 0b00101001)  # MODU64 @R1, R2
        # lower(bc, 0x13, False, True, 0b10100001)  # MODU64 R1, @R2
        # lower(bc, 0x13, False, True, 0b10101001)  # MODU64 @R1, @R2
        # lower(bc, 0x13, True, True, 0b00100001)  # MODU64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x13, True, True, 0b10100001)  # MODU64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x13, True, True, 0b10101001)  # MODU64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x17, False, False, 0b00100001)  # SHL32 R1, R2
        # lower(bc, 0x17, False, False, 0b00101001)  # SHL32 @R1, R2
        # lower(bc, 0x17, False, False, 0b10100001)  # SHL32 R1, @R2
        # lower(bc, 0x17, False, False, 0b10101001)  # SHL32 @R1, @R2
        # lower(bc, 0x17, True, False, 0b00100001)  # SHL32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x17, True, False, 0b10100001)  # SHL32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x17, True, False, 0b10101001)  # SHL32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x17, False, True, 0b00100001)  # SHL64 R1, R2
        # lower(bc, 0x17, False, True, 0b00101001)  # SHL64 @R1, R2
        # lower(bc, 0x17, False, True, 0b10100001)  # SHL64 R1, @R2
        # lower(bc, 0x17, False, True, 0b10101001)  # SHL64 @R1, @R2
        # lower(bc, 0x17, True, True, 0b00100001)  # SHL64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x17, True, True, 0b10100001)  # SHL64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x17, True, True, 0b10101001)  # SHL64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x18, False, False, 0b00100001)  # SHR32 R1, R2
        # lower(bc, 0x18, False, False, 0b00101001)  # SHR32 @R1, R2
        # lower(bc, 0x18, False, False, 0b10100001)  # SHR32 R1, @R2
        # lower(bc, 0x18, False, False, 0b10101001)  # SHR32 @R1, @R2
        # lower(bc, 0x18, True, False, 0b00100001)  # SHR32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x18, True, False, 0b10100001)  # SHR32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x18, True, False, 0b10101001)  # SHR32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x18, False, True, 0b00100001)  # SHR64 R1, R2
        # lower(bc, 0x18, False, True, 0b00101001)  # SHR64 @R1, R2
        # lower(bc, 0x18, False, True, 0b10100001)  # SHR64 R1, @R2
        # lower(bc, 0x18, False, True, 0b10101001)  # SHR64 @R1, @R2
        # lower(bc, 0x18, True, True, 0b00100001)  # SHR64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x18, True, True, 0b10100001)  # SHR64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x18, True, True, 0b10101001)  # SHR64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x0D, False, False, 0b00100001)  # SUB32 R1, R2
        # lower(bc, 0x0D, False, False, 0b00101001)  # SUB32 @R1, R2
        # lower(bc, 0x0D, False, False, 0b10100001)  # SUB32 R1, @R2
        # lower(bc, 0x0D, False, False, 0b10101001)  # SUB32 @R1, @R2
        # lower(bc, 0x0D, True, False, 0b00100001)  # SUB32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x0D, True, False, 0b10100001)  # SUB32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x0D, True, False, 0b10101001)  # SUB32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x0D, False, True, 0b00100001)  # SUB64 R1, R2
        # lower(bc, 0x0D, False, True, 0b00101001)  # SUB64 @R1, R2
        # lower(bc, 0x0D, False, True, 0b10100001)  # SUB64 R1, @R2
        # lower(bc, 0x0D, False, True, 0b10101001)  # SUB64 @R1, @R2
        # lower(bc, 0x0D, True, True, 0b00100001)  # SUB64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x0D, True, True, 0b10100001)  # SUB64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x0D, True, True, 0b10101001)  # SUB64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x16, False, False, 0b00100001)  # XOR32 R1, R2
        # lower(bc, 0x16, False, False, 0b00101001)  # XOR32 @R1, R2
        # lower(bc, 0x16, False, False, 0b10100001)  # XOR32 R1, @R2
        # lower(bc, 0x16, False, False, 0b10101001)  # XOR32 @R1, @R2
        # lower(bc, 0x16, True, False, 0b00100001)  # XOR32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x16, True, False, 0b10100001)  # XOR32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x16, True, False, 0b10101001)  # XOR32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x16, False, True, 0b00100001)  # XOR64 R1, R2
        # lower(bc, 0x16, False, True, 0b00101001)  # XOR64 @R1, R2
        # lower(bc, 0x16, False, True, 0b10100001)  # XOR64 R1, @R2
        # lower(bc, 0x16, False, True, 0b10101001)  # XOR64 @R1, @R2
        # lower(bc, 0x16, True, True, 0b00100001)  # XOR64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x16, True, True, 0b10100001)  # XOR64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x16, True, True, 0b10101001)  # XOR64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x05, False, False, 0b00100001)  # CMPeq32 R1, R2
        # lower(bc, 0x05, False, False, 0b00101001)  # CMPeq32 @R1, R2
        # lower(bc, 0x05, False, False, 0b10100001)  # CMPeq32 R1, @R2
        # lower(bc, 0x05, False, False, 0b10101001)  # CMPeq32 @R1, @R2
        # lower(bc, 0x05, True, False, 0b00100001)  # CMPeq32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x05, True, False, 0b10100001)  # CMPeq32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x05, True, False, 0b10101001)  # CMPeq32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x05, False, True, 0b00100001)  # CMPeq64 R1, R2
        # lower(bc, 0x05, False, True, 0b00101001)  # CMPeq64 @R1, R2
        # lower(bc, 0x05, False, True, 0b10100001)  # CMPeq64 R1, @R2
        # lower(bc, 0x05, False, True, 0b10101001)  # CMPeq64 @R1, @R2
        # lower(bc, 0x05, True, True, 0b00100001)  # CMPeq64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x05, True, True, 0b10100001)  # CMPeq64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x05, True, True, 0b10101001)  # CMPeq64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x06, False, False, 0b00100001)  # CMPlte32 R1, R2
        # lower(bc, 0x06, False, False, 0b00101001)  # CMPlte32 @R1, R2
        # lower(bc, 0x06, False, False, 0b10100001)  # CMPlte32 R1, @R2
        # lower(bc, 0x06, False, False, 0b10101001)  # CMPlte32 @R1, @R2
        # lower(bc, 0x06, True, False, 0b00100001)  # CMPlte32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x06, True, False, 0b10100001)  # CMPlte32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x06, True, False, 0b10101001)  # CMPlte32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x06, False, True, 0b00100001)  # CMPlte64 R1, R2
        # lower(bc, 0x06, False, True, 0b00101001)  # CMPlte64 @R1, R2
        # lower(bc, 0x06, False, True, 0b10100001)  # CMPlte64 R1, @R2
        # lower(bc, 0x06, False, True, 0b10101001)  # CMPlte64 @R1, @R2
        # lower(bc, 0x06, True, True, 0b00100001)  # CMPlte64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x06, True, True, 0b10100001)  # CMPlte64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x06, True, True, 0b10101001)  # CMPlte64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x07, False, False, 0b00100001)  # CMPgte32 R1, R2
        # lower(bc, 0x07, False, False, 0b00101001)  # CMPgte32 @R1, R2
        # lower(bc, 0x07, False, False, 0b10100001)  # CMPgte32 R1, @R2
        # lower(bc, 0x07, False, False, 0b10101001)  # CMPgte32 @R1, @R2
        # lower(bc, 0x07, True, False, 0b00100001)  # CMPgte32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x07, True, False, 0b10100001)  # CMPgte32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x07, True, False, 0b10101001)  # CMPgte32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x07, False, True, 0b00100001)  # CMPgte64 R1, R2
        # lower(bc, 0x07, False, True, 0b00101001)  # CMPgte64 @R1, R2
        # lower(bc, 0x07, False, True, 0b10100001)  # CMPgte64 R1, @R2
        # lower(bc, 0x07, False, True, 0b10101001)  # CMPgte64 @R1, @R2
        # lower(bc, 0x07, True, True, 0b00100001)  # CMPgte64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x07, True, True, 0b10100001)  # CMPgte64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x07, True, True, 0b10101001)  # CMPgte64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x08, False, False, 0b00100001)  # CMPulte32 R1, R2
        # lower(bc, 0x08, False, False, 0b00101001)  # CMPulte32 @R1, R2
        # lower(bc, 0x08, False, False, 0b10100001)  # CMPulte32 R1, @R2
        # lower(bc, 0x08, False, False, 0b10101001)  # CMPulte32 @R1, @R2
        # lower(bc, 0x08, True, False, 0b00100001)  # CMPulte32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x08, True, False, 0b10100001)  # CMPulte32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x08, True, False, 0b10101001)  # CMPulte32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x08, False, True, 0b00100001)  # CMPulte64 R1, R2
        # lower(bc, 0x08, False, True, 0b00101001)  # CMPulte64 @R1, R2
        # lower(bc, 0x08, False, True, 0b10100001)  # CMPulte64 R1, @R2
        # lower(bc, 0x08, False, True, 0b10101001)  # CMPulte64 @R1, @R2
        # lower(bc, 0x08, True, True, 0b00100001)  # CMPulte64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x08, True, True, 0b10100001)  # CMPulte64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x08, True, True, 0b10101001)  # CMPulte64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x09, False, False, 0b00100001)  # CMPugte32 R1, R2
        # lower(bc, 0x09, False, False, 0b00101001)  # CMPugte32 @R1, R2
        # lower(bc, 0x09, False, False, 0b10100001)  # CMPugte32 R1, @R2
        # lower(bc, 0x09, False, False, 0b10101001)  # CMPugte32 @R1, @R2
        # lower(bc, 0x09, True, False, 0b00100001)  # CMPugte32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x09, True, False, 0b10100001)  # CMPugte32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x09, True, False, 0b10101001)  # CMPugte32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x09, False, True, 0b00100001)  # CMPugte64 R1, R2
        # lower(bc, 0x09, False, True, 0b00101001)  # CMPugte64 @R1, R2
        # lower(bc, 0x09, False, True, 0b10100001)  # CMPugte64 R1, @R2
        # lower(bc, 0x09, False, True, 0b10101001)  # CMPugte64 @R1, @R2
        # lower(bc, 0x09, True, True, 0b00100001)  # CMPugte64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x09, True, True, 0b10100001)  # CMPugte64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x09, True, True, 0b10101001)  # CMPugte64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x0E, False, False, 0b00100001)  # MUL32 R1, R2
        # lower(bc, 0x0E, False, False, 0b00101001)  # MUL32 @R1, R2
        # lower(bc, 0x0E, False, False, 0b10100001)  # MUL32 R1, @R2
        # lower(bc, 0x0E, False, False, 0b10101001)  # MUL32 @R1, @R2
        # lower(bc, 0x0E, True, False, 0b00100001)  # MUL32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x0E, True, False, 0b10100001)  # MUL32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x0E, True, False, 0b10101001)  # MUL32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x0E, False, True, 0b00100001)  # MUL64 R1, R2
        # lower(bc, 0x0E, False, True, 0b00101001)  # MUL64 @R1, R2
        # lower(bc, 0x0E, False, True, 0b10100001)  # MUL64 R1, @R2
        # lower(bc, 0x0E, False, True, 0b10101001)  # MUL64 @R1, @R2
        # lower(bc, 0x0E, True, True, 0b00100001)  # MUL64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x0E, True, True, 0b10100001)  # MUL64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x0E, True, True, 0b10101001)  # MUL64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x0F, False, False, 0b00100001)  # MULU32 R1, R2
        # lower(bc, 0x0F, False, False, 0b00101001)  # MULU32 @R1, R2
        # lower(bc, 0x0F, False, False, 0b10100001)  # MULU32 R1, @R2
        # lower(bc, 0x0F, False, False, 0b10101001)  # MULU32 @R1, @R2
        # lower(bc, 0x0F, True, False, 0b00100001)  # MULU32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x0F, True, False, 0b10100001)  # MULU32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x0F, True, False, 0b10101001)  # MULU32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x0F, False, True, 0b00100001)  # MULU64 R1, R2
        # lower(bc, 0x0F, False, True, 0b00101001)  # MULU64 @R1, R2
        # lower(bc, 0x0F, False, True, 0b10100001)  # MULU64 R1, @R2
        # lower(bc, 0x0F, False, True, 0b10101001)  # MULU64 @R1, @R2
        # lower(bc, 0x0F, True, True, 0b00100001)  # MULU64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x0F, True, True, 0b10100001)  # MULU64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x0F, True, True, 0b10101001)  # MULU64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x0B, False, False, 0b00100001)  # NEG32 R1, R2
        # lower(bc, 0x0B, False, False, 0b00101001)  # NEG32 @R1, R2
        # lower(bc, 0x0B, False, False, 0b10100001)  # NEG32 R1, @R2
        # lower(bc, 0x0B, False, False, 0b10101001)  # NEG32 @R1, @R2
        # lower(bc, 0x0B, True, False, 0b00100001)  # NEG32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x0B, True, False, 0b10100001)  # NEG32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x0B, True, False, 0b10101001)  # NEG32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x0B, False, True, 0b00100001)  # NEG64 R1, R2
        # lower(bc, 0x0B, False, True, 0b00101001)  # NEG64 @R1, R2
        # lower(bc, 0x0B, False, True, 0b10100001)  # NEG64 R1, @R2
        # lower(bc, 0x0B, False, True, 0b10101001)  # NEG64 @R1, @R2
        # lower(bc, 0x0B, True, True, 0b00100001)  # NEG64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x0B, True, True, 0b10100001)  # NEG64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x0B, True, True, 0b10101001)  # NEG64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x0A, False, False, 0b00100001)  # NOT32 R1, R2
        # lower(bc, 0x0A, False, False, 0b00101001)  # NOT32 @R1, R2
        # lower(bc, 0x0A, False, False, 0b10100001)  # NOT32 R1, @R2
        # lower(bc, 0x0A, False, False, 0b10101001)  # NOT32 @R1, @R2
        # lower(bc, 0x0A, True, False, 0b00100001)  # NOT32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x0A, True, False, 0b10100001)  # NOT32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x0A, True, False, 0b10101001)  # NOT32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x0A, False, True, 0b00100001)  # NOT64 R1, R2
        # lower(bc, 0x0A, False, True, 0b00101001)  # NOT64 @R1, R2
        # lower(bc, 0x0A, False, True, 0b10100001)  # NOT64 R1, @R2
        # lower(bc, 0x0A, False, True, 0b10101001)  # NOT64 @R1, @R2
        # lower(bc, 0x0A, True, True, 0b00100001)  # NOT64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x0A, True, True, 0b10100001)  # NOT64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x0A, True, True, 0b10101001)  # NOT64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x15, False, False, 0b00100001)  # OR32 R1, R2
        # lower(bc, 0x15, False, False, 0b00101001)  # OR32 @R1, R2
        # lower(bc, 0x15, False, False, 0b10100001)  # OR32 R1, @R2
        # lower(bc, 0x15, False, False, 0b10101001)  # OR32 @R1, @R2
        # lower(bc, 0x15, True, False, 0b00100001)  # OR32 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x15, True, False, 0b10100001)  # OR32 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x15, True, False, 0b10101001)  # OR32 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        # lower(bc, 0x15, False, True, 0b00100001)  # OR64 R1, R2
        # lower(bc, 0x15, False, True, 0b00101001)  # OR64 @R1, R2
        # lower(bc, 0x15, False, True, 0b10100001)  # OR64 R1, @R2
        # lower(bc, 0x15, False, True, 0b10101001)  # OR64 @R1, @R2
        # lower(bc, 0x15, True, True, 0b00100001)  # OR64 R1, R2 -1000
        # arg(bc, -1000, 2, 'little', signed=True)
        # lower(bc, 0x15, True, True, 0b10100001)  # OR64 R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')
        # lower(bc, 0x15, True, True, 0b10101001)  # OR64 @R1, @R2(-3, -3)
        # arg(bc, NATIND16, 2, 'little')

        return

        ops = dict(
            ADD=0x0C,
            AND=0x14,
            ASHR=0x19,
            DIV=0x10,
            DIVU=0x11,
            EXTNDB=0x1A,
            EXTNDD=0x1C,
            EXTNDW=0x1B,
            MOD=0x12,
            MODU=0x13,
            SHL=0x17,
            SHR=0x18,
            SUB=0x0D,
            XOR=0x16,
            CMPeq=0x05,
            CMPlte=0x06,
            CMPgte=0x07,
            CMPulte=0x08,
            CMPugte=0x09,
            MUL=0x0E,
            MULU=0x0F,

            NEG=0x0B,
            NOT=0x0A,
            OR=0x15
        )

        for op in ops.values():
            # print(str(op).ljust(3), bin(op)[2:].ljust(10), pad(6, bin(op)[2:]))

            # <OPCODE>32 OP1, OP2
            opcode = op
            bc.write(opcode.to_bytes(1, 'little'))
            bc.write(0b00100001.to_bytes(1, 'little'))

            # <OPCODE>32 @OP1, OP2
            opcode = op
            bc.write(opcode.to_bytes(1, 'little'))
            bc.write(0b00101001.to_bytes(1, 'little'))

            # <OPCODE>32 OP1, @OP2
            opcode = op
            bc.write(opcode.to_bytes(1, 'little'))
            bc.write(0b10100001.to_bytes(1, 'little'))

            # <OPCODE>32 @OP1, @OP2
            opcode = op
            bc.write(opcode.to_bytes(1, 'little'))
            bc.write(0b10101001.to_bytes(1, 'little'))

            # <OPCODE>32 OP1, OP2 IMMEDIATE
            opcode = op | (1 << 7)  # Set 8th bit
            opcode &= ~(1 << 6)  # Clear 7th bit
            bc.write(opcode.to_bytes(1, 'little'))
            bc.write(0b00100001.to_bytes(1, 'little'))
            bc.write(next(random_signed).to_bytes(2, 'big', signed=True))

            # <OPCODE>32 OP1, OP2 IMMEDIATE
            opcode = op | (1 << 7)  # Set 8th bit
            opcode &= ~(1 << 6)  # Clear 7th bit
            bc.write(opcode.to_bytes(1, 'little'))
            bc.write(0b10100001.to_bytes(1, 'little'))
            natural = next(random_signed_20)
            if natural < 0:
                constant = -abs(next(random_signed_20))
            else:
                constant = abs(next(random_signed_20))
            index = encode(natural, constant, False)
            bc.write(index.to_bytes(2, 'little'))

            # <OPCODE>64 OP1, OP2
            opcode = op
            opcode |= 1 << 6
            bc.write(opcode.to_bytes(1, 'little'))
            bc.write(0b00100001.to_bytes(1, 'little'))

            # <OPCODE>64 OP1, OP2
            opcode = op
            opcode |= 1 << 6
            bc.write(opcode.to_bytes(1, 'little'))
            bc.write(0b00101001.to_bytes(1, 'little'))

            # <OPCODE>64 OP1, OP2 IMMEDIATE
            opcode = op | (1 << 7)  # Set 8th bit
            opcode |= 1 << 6  # Set 7th bit
            bc.write(opcode.to_bytes(1, 'little'))
            bc.write(0b00100001.to_bytes(1, 'little'))
            bc.write(next(random_signed).to_bytes(2, 'big', signed=True))

            # <OPCODE>64 OP1, OP2 IMMEDIATE
            opcode = op | (1 << 7)  # Set 8th bit
            opcode |= 1 << 6  # Set 7th bit
            bc.write(opcode.to_bytes(1, 'little'))
            bc.write(0b10100001.to_bytes(1, 'little'))
            natural = next(random_signed_20)
            if natural < 0:
                constant = -abs(next(random_signed_20))
            else:
                constant = abs(next(random_signed_20))
            index = encode(natural, constant, False)
            bc.write(index.to_bytes(2, 'little'))


write_bytecode()
