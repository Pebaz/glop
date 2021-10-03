import sys

def _bit_int(bits):
    return int(''.join(str(i) for i in bits), base=2)

def encode(nat, const):
    pass

def decode(index, index_size):
    natural_index_encoding_sizes = {16: 2, 32: 4, 64: 8}

    assert index_size in natural_index_encoding_sizes, (
        'index_size must be one of: 16, 32, 64'
    )

    HEADER_SIZE = 4  # Sign bit + natural bits size
    MACHINE_ARCHITECTURE = 64  # * Assuming 64-bit here

    bits = [int(bit) for bit in bin(index)[2:]]
    sign = bits[0]
    nat_bits = _bit_int(bits[1:4])
    nat_size = nat_bits * natural_index_encoding_sizes[index_size]
    const_size = index_size - HEADER_SIZE - nat_size
    constant = _bit_int(bits[HEADER_SIZE:HEADER_SIZE + const_size])
    natural = _bit_int(bits[-nat_size:])
    offset = sign * (constant + natural * (MACHINE_ARCHITECTURE // 8))

    print('Sign:', 'negative' if sign else 'positive')
    print('Constant:', constant)
    print('Natural Units:', natural)
    print('Offset:', offset)


if __name__ == '__main__':
    if len(sys.argv) != 4:
        sys.exit(
            print(
                'Usage:\n'
                '  natural_indexing ENCODE <CONSTANT> <NATURAL>\n'
                '  natural_indexing DECODE <NATURAL INDEX> <INDEX SIZE>'
            )
        )

    _, cmd, *args = sys.argv

    if cmd == 'ENCODE':
        encode(*(int(i) for i in args))

    elif cmd == 'DECODE':
        decode(*(int(i) for i in args))
