include 'ebc.inc'
include 'efi.inc'
include 'format.inc'
include 'utf8.inc'

format peebc efi  ;; PE executable format, EFI Byte Code

entry efi_main

section '.text' code executable readable

efi_main:
    MOVn   R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)
    MOVn   R1, @R1(EFI_SYSTEM_TABLE.ConOut)

    ;; Add 2 numbers
    MOVI R3, 123
    MOVI R4, 1
    ADD R3, R4

    ;; Compare them, if they are equal, set condition code (CS)
    CMPIeq R3, 124
    JMPcc else_block  ;; If not equal and condition cleared (CC), jump
    MOVREL R2, string_succeed  ;; Continue onto truthy block
    JMP print_result

    ;; RET -> Never returns

else_block:
    MOVREL R2, string_failed  ;; Falsey block
    JMP print_result

print_result:
    PUSHn  R2
    PUSHn  R1
    CALLEX @R1(SIMPLE_TEXT_OUTPUT_INTERFACE.OutputString)
    MOV R0, R0(+2,0)
    JMP efi_main

section '.data' data readable writeable
    string_succeed: du "YES", 0x0A, 0x00
    string_failed: du "NO", 0x0A, 0x00
    number_a: dq 123
    number_b: dq 1
    result: dq 124
