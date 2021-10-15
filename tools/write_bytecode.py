with open('bc-example.bin', 'wb') as bc:
    bc.write(0b00000100.to_bytes(1, 'big'))  # RET

    bc.write(0b0000110000100001.to_bytes(2, 'big'))  # ADD32 R1, R2
