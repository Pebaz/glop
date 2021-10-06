"""
UEFI.22.8
"""

# TODO(pbz): Gotta check what the effect of little-endian has on all this.
REGISTERS = R0, R1, R2, R3, R4, R5, R6, R7 = range(8)

def to_bits(num: int, bit_width=None):
    bits = [int(bit) for bit in bin(num)[2:]]

    if bit_width and bit_width > len(bits):
        bits = [0] * (bit_width - len(bits)) + bits

    return bits


# @instruction(0x0C)
# func.__opcode__ = opcode
# func.__opcode_str__ = 'ADD64...'
def add(op1, op2, immediate=None, index=None, x64=True):
    bits = []

    # Byte 1
    bits.append(int(bool(immediate or index)))
    bits.append(int(bool(x64)))
    bits.extend(to_bits(0x0C, bit_width=6))

    # Byte 2
    bits.append(0)  # Assume direct
    bits.extend(to_bits(op2, bit_width=3))
    bits.append(0)
    bits.extend(to_bits(op1, bit_width=3))

    # Byte 3
    "Ignored for now"

    # Byte 4
    "Ignored for now"

    return bits


def which_instruction(instruction):
    # print(instruction.__opcode__)
    pass

print(add(R7, R1))
print(len(add(R7, R1)))
