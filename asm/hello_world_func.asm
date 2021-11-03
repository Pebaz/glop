include 'ebc.inc'
include 'efi.inc'
include 'format.inc'
include 'utf8.inc'

format peebc efi  ; PE executable format, EFI Byte Code

entry efi_main

section '.text' code executable readable

Print:
    MOVREL    R1, gST
    MOV       R1, @R1
    MOVn      R1, @R1(EFI_SYSTEM_TABLE.ConOut)
    PUSHn     @R0(0,+16)
    PUSHn     R1
    CALLEX    @R1(SIMPLE_TEXT_OUTPUT_INTERFACE.OutputString)
    MOV       R0, R0(+2,0)
    RET

efi_main:
    XOR       R6, R6
    MOVREL    R1, gST
    MOVn      @R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)

    ;; Push a different message based on an if statement
    MOVI R3, 124

    CMPIeq R3, 123
    JMPcc else_block  ;; If not equal and condition cleared (CC), jump

    MOVREL R1, string_succeed  ;; Continue onto truthy block
    JMP continue

else_block:
    MOVREL R1, string_failed  ;; Continue onto truthy block
    JMP continue

continue:
    ;; MOVREL    R1, EpMsg
    PUSH      R1
    CALL      Print
    POP       R1
    JMP efi_main
    RET



section '.data' data readable writeable
    gST:      dq ?  ;; ! http://flatassembler.net/docs.php?article=fasmg_manual
    EpMsg:    du "Entry point: ", 0x00
    string_hello: du "Hello World!", 0x0A, 0x0
    string_succeed: du "YES", 0x0A, 0x00
    string_failed: du "NO", 0x0A, 0x00
