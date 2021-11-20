include 'ebc.inc'
include 'efi.inc'
include 'format.inc'
include 'utf8.inc'

format peebc efi  ; PE executable format, EFI Byte Code

entry efi_main

section '.text' code executable readable

Print:
    MOVREL    R1, system_table
    MOV       R1, @R1
    MOVn      R1, @R1(EFI_SYSTEM_TABLE.ConOut)
    PUSHn     @R0(0,+16)
    PUSHn     R1
    CALLEX    @R1(SIMPLE_TEXT_OUTPUT_INTERFACE.OutputString)
    MOV       R0, R0(+2,0)
    RET

efi_main:
    MOVREL    R1, system_table  ;; Move system_table into R1
    MOVn      @R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)

    ;; Push a different message based on an if statement
    MOVI R3, 124

    CMPIeq R3, 124
    JMPcc else_block  ;; If not equal and condition cleared (CC), jump

    MOVREL R1, string_succeed  ;; Continue onto truthy block
    JMP continue

else_block:
    MOVREL R1, string_failed  ;; Continue onto truthy block
    JMP continue

continue:
    PUSH      R1
    CALL      Print
    POP       R1
    JMP efi_main
    RET

;; ! http://flatassembler.net/docs.php?article=fasmg_manual
section '.data' data readable writeable
    ;; Reserve 8 bytes in the .data section to use for storing a pointer
    system_table: dq ?
    string_succeed: du "YES", 0x0A, 0x00
    string_failed: du "NO", 0x0A, 0x00
