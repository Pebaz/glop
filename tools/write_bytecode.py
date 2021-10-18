import random
from itertools import cycle

RANDOM_SIGNED_NUMBERS_X16 = [random.randint(-32768, 32767) for i in range(100)]
random_signed = cycle(iter(RANDOM_SIGNED_NUMBERS_X16))


def pad(width, string):
    return '0' * (width - len(string)) + string

with open('bc-example.bin', 'wb') as bc:
    bc.write(0b00000100.to_bytes(1, 'big'))  # RET

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
        CMPIeq=0x2D,
        CMPIlte=0x2E,
        CMPIgte=0x2F,
        CMPIulte=0x30,
        CMPIugte=0x31,
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

        # <OPCODE>32 OP1, OP2 IMMEDIATE
        opcode = op | (1 << 7)  # Set 8th bit
        opcode &= ~(1 << 6)  # Clear 7th bit
        bc.write(opcode.to_bytes(1, 'little'))
        bc.write(0b00100001.to_bytes(1, 'little'))
        bc.write(next(random_signed).to_bytes(2, 'big', signed=True))

        # <OPCODE>64 OP1, OP2
        opcode = op
        opcode |= 1 << 6
        bc.write(opcode.to_bytes(1, 'little'))
        bc.write(0b00100001.to_bytes(1, 'little'))

        # <OPCODE>64 OP1, OP2 IMMEDIATE
        opcode = op | (1 << 7)  # Set 8th bit
        opcode |= 1 << 6  # Set 7th bit
        print(bin(opcode))
        bc.write(opcode.to_bytes(1, 'little'))
        bc.write(0b00100001.to_bytes(1, 'little'))
        bc.write(next(random_signed).to_bytes(2, 'big', signed=True))
