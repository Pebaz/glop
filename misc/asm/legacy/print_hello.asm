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

; ORIGINAL WORKING MAIN FUNCTION
; efi_main:
;     MOVn   R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)
;     MOVn   R1, @R1(EFI_SYSTEM_TABLE.ConOut)
;     MOVREL R2, string_hello
;     PUSHn  R2
;     PUSHn  R1
;     CALLEX @R1(SIMPLE_TEXT_OUTPUT_INTERFACE.OutputString)
;     MOV R0, R0(+2,0)
;     JMP efi_main
;     RET

;; void print(EFI_SYSTEM_TABLE * SystemTable);
print:
    ; MOVn R1, @R0(+1, 0)
    POP R1
    MOVn R4, @R1(EFI_SYSTEM_TABLE.ConOut)
    MOVn R3, @R4(SIMPLE_TEXT_OUTPUT_INTERFACE.OutputString)
    MOVREL R2, string_hello

    ;; Push parameters onto native stack in reverse order
    PUSHn R2
    PUSHn R4
    CALLEX @R3
    MOV R0, R0(+2, 0)  ;; POP, POP
    PUSH R1
    RET


;; ! WE ARE LOSING THE POINTER TO EFI_SYSTEM_TABLE!!!!!
;; EFI_STATUS EfiMain(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE * SystemTable);
;; Before we modify the stack again, we need to preserve the EFI_SYSTEM_TABLE
;; This gets passed in: EFI_MAIN_PARAMETERS
efi_main:
    MOVn R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)
    MOVn R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)

    ;; Using normal PUSH here since it is used by print()
    PUSHn R1  ;; Keep track of EFI_SYSTEM_TABLE
loop1:
    ;; As long as the stack doesn't get modified, we can rely on @RO(SysTable)
    PUSH @R0(+1, 0)  ;; Push the EFI_SYSTEM_TABLE pointer for use by print()
    CALL print
    POP R6  ;; Not using R6
    JMP loop1



section '.data' data readable writeable
    string_hello: du "Hello World!", 0x0A, 0x00
