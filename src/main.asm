include 'ebc.inc'
include 'efi.inc'
include 'format.inc'
include 'utf8.inc'

format peebc efi  ; PE executable format, EFI Byte Code

entry efi_main

section '.text' code executable readable

efi_main:
    MOVn   R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)
    MOVn   R1, @R1(EFI_SYSTEM_TABLE.ConOut)
    MOVREL R2, string_hello
    PUSHn  R2
    PUSHn  R1
    CALLEX @R1(SIMPLE_TEXT_OUTPUT_INTERFACE.OutputString)
    MOV R0, R0(+2,0)
    JMP efi_main
    RET


section '.data' data readable writeable
    string_hello: du "Hello World!", 0x0A


; 16 Bit Natural Index: 1_010_00000100_1000
1 Sign Bit -> 1
3 Natural units size * 2 (since this is a 16 bit natural index)
8 Natural bits (4 from above minus 12 bits left after initial 4 = 8)
4 Constant bits (12 bits left minus 8 natural bits)
