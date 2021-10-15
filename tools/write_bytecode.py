with open('bc-example.bin', 'wb') as bc:
    bc.write(0b00000100.to_bytes(1, 'big'))  # RET

    bc.write(0b00000100.to_bytes(1, 'big'))  # RET
