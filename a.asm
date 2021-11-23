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
    JMP32 CLEARSCREEN

    STORESP R6, [IP]
    JMP32 EMITSTR

    MOVREL R1, const_u64_0
    PUSH R1
    MOVREL R1, const_u64_1
    PUSH R1
    MOVREL R1, const_u64_2
    PUSH R1
    MOVREL R1, const_u64_3
    PUSH R1
    MOVREL R1, const_u64_4
    PUSH R1
    STORESP R6, [IP]
    JMP32 DRAWPIXEL
