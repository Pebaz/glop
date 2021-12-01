from string import ascii_lowercase

SYMBOL_CHARS = ascii_lowercase + {'-'}
EMIT_U64 = '''

'''

def assemble(code):
    # @emit-u64(123u64)
    assert code.startswith('@'), 'Only assembly routines "@" are supported'

    assembly_routine_name = []
    u64_code = []

    for c in code[1:]:
        if c in SYMBOL_CHARS:
            assembly_routine_name.append(c)
        elif c == '(':
            u64_code.append(c)
        elif c == ')':
            break

    assembly_routine_name = ''.join(assembly_routine_name)
    u64_code = ''.join(u64_code)

    assert assembly_routine_name
    assert u64_code.endswith('u64')
    u64_code.replace('u64', '')
    assert u64_code

    if assembly_routine_name == 'emit-u64':
        print(EMIT_U64.format(u64_code))
