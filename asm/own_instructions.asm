include 'ebc.inc'
include 'efi.inc'
include 'format.inc'
include 'utf8.inc'

format peebc efi  ; PE executable format, EFI Byte Code

entry efi_main

;; This is for all the assembly for the whole program
section 'CODE' code executable readable

include 'instructions.inc'

;; Not yet. All code is global in this file.
; fn_entry:
;     JMP32 R6(+0, +2)

efi_main:
    ;; First order of business, store the pointer to the system table
    MOVREL R1, system_table
    MOVn @R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)

    ;; BEGIN OWN INSTRUCTIONS

    STORESP R6, [IP]
    JMP CLEARSCREEN

    MOVREL R1, string_hello_world
    PUSH R1
    STORESP R6, [IP]
    JMP EMITSTR

    ;; END OWN INSTRUCTIONS

    looop:
        JMP looop
    RET


;; This is for uninitialized global variables and is used in leu of malloc
section 'RESERVED' data readable writeable
    system_table: dq ?

;; This is for initialized global variables
section 'DATA' data readable writeable
    string_hello_world: du "Hello World!", 0x0D, 0x0A, 0x00
