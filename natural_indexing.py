import sys

MACHINE_ARCHITECTURE = 64  # * Assuming 64-bit here
SIZE_OF_VOID_PTR = MACHINE_ARCHITECTURE // 8
S_BITS = 1  # Sign bits
W_BITS = 3  # Natural bit length
HEADER_SIZE = S_BITS + W_BITS
NATURAL_INDEX_ENCODING_SIZES = {16: 2, 32: 4, 64: 8}

def _bit_int(bits):
    if not bits:
        return 0
    return int(''.join(str(i) for i in bits), base=2)

# 0xA048 = 41032
def encode(natural, constant):
    # s = 0
    # w = 000
    # a = w * 2 (from 16)
    # n = last a bits
    # c = rest of middle bits

    assert (
        natural >= 0 and constant >= 0) or (natural < 0 and constant < 0
    ), 'Natural and constant index components must have same sign.'

    bits = []
    bits.append(int(natural < 0))

    natural, constant = abs(natural), abs(constant)

    nat_bits = [int(bit) for bit in bin(natural)[2:]]
    if nat_bits == [0]:
        nat_bits = []

    const_bits = [int(bit) for bit in bin(constant)[2:]]
    if const_bits == [0]:
        const_bits = []

    index_size = 16
    if (len(nat_bits) + len(const_bits)) >= index_size - HEADER_SIZE:
        index_size = 32
    if (len(nat_bits) + len(const_bits)) >= index_size - HEADER_SIZE:
        index_size = 64

    index_sizer = NATURAL_INDEX_ENCODING_SIZES[index_size]

    # Find size of natural index
    for w in range(7):
        a = w * index_sizer
        if a >= len(nat_bits):
            break
    else:
        raise Exception('Did not find enough room to store natural index')

    w_bits = [int(bit) for bit in bin(w)[2:]]
    w_bits = [0] * (W_BITS - len(w_bits)) + w_bits
    bits.extend(w_bits)

    constant_bits_left = index_size - HEADER_SIZE - a
    const_bits = [0] * (constant_bits_left - len(const_bits)) + const_bits
    bits.extend(const_bits)

    nat_bits = [0] * (a - len(nat_bits)) + nat_bits
    bits.extend(nat_bits)
    sign = -1 if bits[0] else 1
    offset = sign * (constant + natural * (SIZE_OF_VOID_PTR))
    col = 16

    print(' ' * (col // 2) + '- Encoded Natural Index -')
    print('Bits:'.rjust(col), bits)
    print('Sign:'.rjust(col), 'negative' if bits[0] else 'positive')
    print('W:'.rjust(col), w)
    print(
        'A:'.rjust(col),
        f'{w} * {NATURAL_INDEX_ENCODING_SIZES[index_size]}'
        f'(x{index_size}) = {a}'
    )
    print('Constant:'.rjust(col), constant)
    print(
        'Natural Units:'.rjust(col),
        natural,
        f'({natural * SIZE_OF_VOID_PTR} bytes, '
        f'{natural * SIZE_OF_VOID_PTR * 8} bits)'
    )
    print('Natural Units:'.rjust(col), natural)
    print('Natural Index:'.rjust(col), _bit_int(bits))
    print('Offset Bytes:'.rjust(col), offset)
    print()

    return _bit_int(bits)

# https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
# Section 2.2.2.2.2.2.
def decode(index, index_size):
    # * Gotta pad the front with more zeros since this is coming in from
    # * Python. This won't happen in reality since it will just be an array of
    # * bytes for the UEFI VM to decode. At worst, 4 zeros will be added to the
    # * front since it could be a positive (0) offset with no natural units (0)
    # * which would result in 4 zeros added to the front: sign (1) + w (3).
    bits = [int(bit) for bit in bin(index)[2:]]
    index_size = 16

    if len(bits) > index_size:
        index_size = 32
    if len(bits) > index_size:
        index_size = 64

    bits = [0] * (index_size - len(bits)) + bits
    sign = -1 if bits[0] else 1
    width_base = _bit_int(bits[1:4])
    actual_width = width_base * NATURAL_INDEX_ENCODING_SIZES[index_size]
    natural = _bit_int(bits[len(bits) - actual_width:])
    constant = _bit_int(bits[4:len(bits) - actual_width])
    offset = sign * (constant + natural * (SIZE_OF_VOID_PTR))
    col = 16

    print(' ' * (col // 2) + '- Decoded Natural Index -')
    print('Bits:'.rjust(col), bits)
    print('Sign:'.rjust(col), 'negative' if sign < 0 else 'positive')
    print('W:'.rjust(col), width_base)
    print(
        'A:'.rjust(col),
        f'{width_base} * {NATURAL_INDEX_ENCODING_SIZES[index_size]}'
        f'(x{index_size}) = {actual_width}'
    )
    print('Constant:'.rjust(col), constant)
    print(
        'Natural Units:'.rjust(col),
        natural,
        f'({natural * SIZE_OF_VOID_PTR} bytes, '
        f'{natural * SIZE_OF_VOID_PTR * 8} bits)'
    )
    print('Natural Index:'.rjust(col), index)
    print('Offset Bytes:'.rjust(col), offset)
    print()


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
        # * Index size arg is ignored to keep same CLI
        decode(encode(*(int(i) for i in args)), None)

    elif cmd.upper() == 'DECODE':
        decode(*(int(i) for i in args))
