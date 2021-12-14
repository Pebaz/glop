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

    ;; This also works. It means "pop 2 items off the stack"
    ;; I believe the + here is because the stack is in the higher address space
    ;; MOV R0, R0(+2,0)

    ;; This pops the 2 arguments used to call OutputString off the stack
    POPn R4  ;; Storing in R4 just for no reason. Better way to do this?
    POPn R4

    JMP efi_main

    ;; Never return
    ;; RET

section '.data' data readable writeable
    string_hello: du "Hello World!", 0x0A, 0x0
