import random
from itertools import cycle
from natural_indexing import encode

RANDOM_SIGNED_NUMBERS_X16 = [random.randint(-32768, 32767) for i in range(100)]
random_signed = cycle(iter(RANDOM_SIGNED_NUMBERS_X16))

RANDOM_SIGNED_NUMBERS_20 = [random.randint(-20, 20) for i in range(100)]
random_signed_20 = cycle(iter(i for i in RANDOM_SIGNED_NUMBERS_20 if i))


def pad(width, string):
    return '0' * (width - len(string)) + string

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

        # # CALL32 @R1(-300, -300)
        # bc.write(0b10000011_00011001.to_bytes(2, 'big'))
        # bc.write((2954019116).to_bytes(4, 'little'))  # ..

        # # CALL32a @R1(-300, -300)
        # bc.write(0b10000011_00001001.to_bytes(2, 'big'))
        # bc.write((2954019116).to_bytes(4, 'little'))  # ..

        # bc.write(0b00000011_00110001.to_bytes(2, 'big'))  # CALL32EX R1
        # bc.write(0b00000011_00100001.to_bytes(2, 'big'))  # CALL32EXa R1

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

        # CMPI
        bc.write(0b00101101_00000001.to_bytes(2, 'big'))  # CMPI32weq R1 1000
        bc.write((1000).to_bytes(2, 'little'))  # ..

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
