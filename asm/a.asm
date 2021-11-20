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

    STORESP R6, [IP]
    JMP CLEARSCREEN

    STORESP R6, [IP]
    JMP CLEARSCREEN



    ;; END OWN INSTRUCTIONS

    looop:
        JMP looop
    RET

;; TODO(pbz): This is temporary. Will want to control this from compiler
;; This is for uninitialized global variables and is used in leu of malloc
section 'RESERVED' data readable writeable
    system_table: dq ?

