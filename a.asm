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
    JMP32 clear-screen

    STORESP R6, [IP]
    JMP32 emit-string

    STORESP R6, [IP]
    JMP32 bu

    STORESP R6, [IP]
    JMP32 baz

    MOVREL R1, const_u64_0
    PUSH R1
    MOVREL R1, const_u64_1
    PUSH R1
    MOVREL R1, const_u64_2
    PUSH R1
    STORESP R6, [IP]
    JMP32 pbz



    ;; END OWN INSTRUCTIONS

    looop:
        JMP looop
    RET

;; TODO(pbz): This is temporary. Will want to control this from compiler
;; This is for uninitialized global variables and is used in leu of malloc
section 'RESERVED' data readable writeable
    system_table: dq ?

;; This is for initialized global variables
section 'DATA' data readable writeable
    const_u64_0: dq 1
    const_u64_1: dq 2
    const_u64_2: dq 3
