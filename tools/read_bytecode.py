import sys

for i, byte_ in enumerate(sys.stdin.buffer.read()):
    if i % 16 == 0:
        print()
    hex_ = hex(byte_)[2:]
    if len(hex_) == 1:
        hex_ = '0' + hex_

    print(hex_, end=' ')

