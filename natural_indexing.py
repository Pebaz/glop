import sys

HEADER_SIZE = 4  # Sign bit + natural bits size

def _bit_int(bits):
    if not bits:
        return 0
    return int(''.join(str(i) for i in bits), base=2)

# 0xA048 = 41032
def encode(natural, constant):
    assert (
        natural >= 0 and constant >= 0) or (natural < 0 and constant < 0
    ), 'Natural and constant index components must have same sign.'

    print(natural, constant)
    bits = []

    bits.append(int(natural < 0))

    natural, constant = abs(natural), abs(constant)

    nat_bits = [int(bit) for bit in bin(natural)[2:]]
    const_bits = [int(bit) for bit in bin(constant)[2:]]

    print([1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0])
    print(nat_bits)
    print(const_bits)

    # ? Just use 64 bit every single time
    nat_ind_enc_size = 2  # x64

    num_natural_bits = 0
    while num_natural_bits * nat_ind_enc_size < len(nat_bits):
        num_natural_bits += 1
    num = bin(num_natural_bits)[2:]
    if len(num) < 3:
        for _ in range(3 - len(num)):
            num = '0' + num
    nat_size = [int(bit) for bit in num]
    bits.extend(nat_size)
    actual_nat_bits = num_natural_bits * nat_ind_enc_size

    # Goal: Add natural bits exactly and then pad only constant bits

    # Add nat bits exactly equal to actual (that 3 bit field)
    while len(nat_bits) < actual_nat_bits:
        nat_bits.insert(0, 0)

    # Now pad constant bits to fill space
    nat_index_width = 16
    while len(const_bits) < nat_index_width - HEADER_SIZE - actual_nat_bits:
        const_bits.insert(0, 0)

    bits.extend(const_bits)
    bits.extend(nat_bits)

    print(bits)
    bits = [*reversed(bits)]
    print('Encoded Offset:', _bit_int(bits))
    return _bit_int(bits)



def decode(index, index_size):
    natural_index_encoding_sizes = {16: 2, 32: 4, 64: 8}

    assert index_size in natural_index_encoding_sizes, (
        'index_size must be one of: 16, 32, 64'
    )

    MACHINE_ARCHITECTURE = 64  # * Assuming 64-bit here

    bits = [int(bit) for bit in bin(index)[2:]]
    bits = [0] * (index_size - len(bits)) + bits

    # s = 0
    # w = 000
    # a = w * 2 (from 16)
    # n = last a bits
    # c = rest of middle bits

    # bits = list(range(16))

    sign = -1 if bits[0] else 1
    width_base = _bit_int(bits[1:4])
    actual_width = width_base * natural_index_encoding_sizes[index_size]
    natural = _bit_int(bits[len(bits) - actual_width:])
    constant = _bit_int(bits[4:len(bits) - actual_width])
    offset = sign * (constant + natural * (MACHINE_ARCHITECTURE // 8))

    # sign = bits[0]
    # nat_bits = _bit_int(bits[1:4])
    # nat_size = nat_bits * natural_index_encoding_sizes[index_size]
    # const_size = index_size - HEADER_SIZE - nat_size
    # constant = _bit_int(bits[HEADER_SIZE:HEADER_SIZE + const_size])
    # natural = _bit_int(bits[-nat_size:])
    # offset = sign * (constant + natural * (MACHINE_ARCHITECTURE // 8))

    col = 16

    print('Bits:'.rjust(col), bits)
    print('Sign:'.rjust(col), 'negative' if sign else 'positive')
    print('W:'.rjust(col), width_base)
    print(
        'A:'.rjust(col),
        f'{width_base} * {natural_index_encoding_sizes[index_size]}'
        f'(x{index_size}) = {actual_width}'
    )
    print('Constant:'.rjust(col), constant)
    print('Natural Units:'.rjust(col), natural)
    print('Offset Bytes:'.rjust(col), offset)

    return

    print('Sign:'.rjust(col), 'negative' if sign else 'positive')
    print('Constant:'.rjust(col), constant)
    print('Natural Units:'.rjust(col), natural)
    print('Offset Bytes:'.rjust(col), offset)


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

    if cmd.upper() == 'ENCODE':
        decode(encode(*(int(i) for i in args)), 64)

    elif cmd.upper() == 'DECODE':
        decode(*(int(i) for i in args))
