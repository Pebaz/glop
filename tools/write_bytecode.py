with open('bc-example.bin', 'wb') as bc:
    bc.write(0b00000100.to_bytes(1, 'big'))  # RET

    bc.write(0b00001100_00100001.to_bytes(2, 'big'))  # ADD32 R1, R2

    # x = (-24).to_bytes(2, 'little', signed=True); bin(x[0]); bin(x[1])
    # ADD32 R1, R2
    bc.write(0b10001100_00100001_11101000_11111111.to_bytes(4, 'big'))
