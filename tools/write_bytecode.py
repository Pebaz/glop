def pad(width, string):
    return '0' * (width - len(string)) + string

with open('bc-example.bin', 'wb') as bc:
    bc.write(0b00000100.to_bytes(1, 'big'))  # RET

    bc.write(0b00001100_00100001.to_bytes(2, 'big'))  # ADD32 R1, R2

    # x = (-24).to_bytes(2, 'little', signed=True); bin(x[0]); bin(x[1])
    # ADD32 R1, R2
    bc.write(0b10001100_00100001_11101000_11111111.to_bytes(4, 'big'))

    bc.write(0b10000101_00100001_11101000_11111111.to_bytes(4, 'big'))
    bc.write(0b10000110_00100001_11101000_11111111.to_bytes(4, 'big'))
    bc.write(0b10000111_00100001_11101000_11111111.to_bytes(4, 'big'))
    bc.write(0b10001000_00100001_11101000_11111111.to_bytes(4, 'big'))
    bc.write(0b10001001_00100001_11101000_11111111.to_bytes(4, 'big'))
    bc.write(0b10101101_00100001_11101000_11111111.to_bytes(4, 'big'))
    bc.write(0b10101110_00100001_11101000_11111111.to_bytes(4, 'big'))
    bc.write(0b10101111_00100001_11101000_11111111.to_bytes(4, 'big'))
    bc.write(0b10110000_00100001_11101000_11111111.to_bytes(4, 'big'))
    bc.write(0b10110001_00100001_11101000_11111111.to_bytes(4, 'big'))
    bc.write(0b10001110_00100001_11101000_11111111.to_bytes(4, 'big'))
    bc.write(0b10001111_00100001_11101000_11111111.to_bytes(4, 'big'))
    bc.write(0b10001011_00100001_11101000_11111111.to_bytes(4, 'big'))
    bc.write(0b10001010_00100001_11101000_11111111.to_bytes(4, 'big'))
    bc.write(0b10010101_00100001_11101000_11111111.to_bytes(4, 'big'))

    # ops = dict(
    #     CMPeq=0x05,
    #     CMPlte=0x06,
    #     CMPgte=0x07,
    #     CMPulte=0x08,
    #     CMPugte=0x09,
    #     CMPIeq=0x2D,
    #     CMPIlte=0x2E,
    #     CMPIgte=0x2F,
    #     CMPIulte=0x30,
    #     CMPIugte=0x31,
    #     MUL=0x0E,
    #     MULU=0x0F,
    #     NEG=0x0B,
    #     NOT=0x0A,
    #     OR=0x15
    # )

    # for op in ops.values():
    #     print(str(op).ljust(3), bin(op)[2:].ljust(10), pad(6, bin(op)[2:]))
