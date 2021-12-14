include 'ebc.inc'
include 'efi.inc'
include 'format.inc'
include 'utf8.inc'

format peebc efi  ;; PE executable format, EFI Byte Code

entry efi_main

section '.text' code executable readable

efi_main:
    MOVI R2, 123
    PUSHn R2
    MOVI R1, 1
    PUSHn R1
    CALL func_add

    MOV R3, R7  ;; Move the return value into R3

    MOVn   R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)
    MOVn   R1, @R1(EFI_SYSTEM_TABLE.ConOut)

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

func_add:
    POP R2  ;; Pop arg2 off stack and store in R2
    POP R1  ;; Pop arg1 off stack and store in R1
    ADD R1, R2
    MOV R7, R1  ;; Move the return value to R7 as per UEFI.22.9.4
    RET

; Hello:
;     MOVREL    R1, string_succeed
;     PUSH      R1
;     CALL      Print
;     POP       R1
;     MOVI      R7, EFI_SUCCESS
;     RET

; Print:
;     MOVREL    R1, string_succeed
;     MOV       R1, @R1
;     MOVn      R1, @R1(EFI_SYSTEM_TABLE.ConOut)
;     PUSHn     @R0(0,+16)
;     PUSHn     R1
;     CALLEX    @R1(SIMPLE_TEXT_OUTPUT_INTERFACE.OutputString)
;     MOV       R0, R0(+2,0)
;     RET


section '.data' data readable writeable
    string_succeed: du "YES", 0x0A, 0x00
    string_failed: du "NO", 0x0A, 0x00
    number: dq 123
    gST:      dq ?
