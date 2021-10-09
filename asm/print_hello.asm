include 'ebc.inc'
include 'efi.inc'
include 'format.inc'
include 'utf8.inc'

format peebc efi  ; PE executable format, EFI Byte Code

entry efi_main

section '.text' code executable readable

;; Original: https://github.com/pbatard/fasmg-ebc/blob/8af690cc918889e1cb36acb6f5d79d7ade7cc455/printhex.asm#L16
; Print:
;     MOVREL    R1, gST
;     MOV       R1, @R1
;     MOVn      R1, @R1(EFI_SYSTEM_TABLE.ConOut)
;     PUSHn     @R0(0,+16)
;     PUSHn     R1  ;; * PUSHn for native calls
;     CALLEX    @R1(SIMPLE_TEXT_OUTPUT_INTERFACE.OutputString)
;     MOV       R0, R0(+2,0)
;     RET

print:
    ; POP R1
    MOVn R1, @R0(+1, 0)
    MOVn R1, @R1(EFI_SYSTEM_TABLE.ConOut)

    MOVREL R2, string_hello
    PUSHn R2

    PUSHn R1

    CALLEX    @R1(SIMPLE_TEXT_OUTPUT_INTERFACE.OutputString)
    MOV       R0, R0(+2, 0)

    RET

;; ! WE ARE LOSING THE POINTER TO EFI_SYSTEM_TABLE!!!!!
;; EFI_STATUS EfiMain(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE * SystemTable);
;; Before we modify the stack again, we need to preserve the EFI_SYSTEM_TABLE
efi_main:
    ; MOVREL R5, string_hello
    ; PUSH R5  ;; This instantly breaks the ConOut stuff in print()

    ;; Push EFI_SYSTEM_TABLE onto the stack
    MOVn R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)
    PUSH R1

    CALL print
    MOV R0, R0(+1, 0)
    POP R1

    JMP efi_main
    RET

section '.data' data readable writeable
    gST: dq ?
    string_hello: du "Hello World!", 0x0A, 0x00
