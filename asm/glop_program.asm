include 'ebc.inc'
include 'efi.inc'
include 'format.inc'
include 'utf8.inc'

format peebc efi  ; PE executable format, EFI Byte Code

entry efi_main

;; This is for uninitialized global variables and is used in leu of malloc
section 'RESERVED' data readable writeable
    system_table: dq ?

;; This is for initialized global variables
section 'DATA' data readable writeable
    string_hello_world: du "Hello World!", 0x0D, 0x0A, 0x00


;; This is for all the assembly for the whole program
section 'CODE' code executable readable


print:
    MOVREL    R1, system_table
    MOV       R1, @R1
    MOVn      R1, @R1(EFI_SYSTEM_TABLE.ConOut)
    PUSHn     @R0(0,+16)
    PUSHn     R1
    CALLEX    @R1(SIMPLE_TEXT_OUTPUT_INTERFACE.OutputString)
    MOV       R0, R0(+2,0)
    RET

; struct fn_hello_world_stack_frame
;     string_hello UINT64  ;; Points to address within DATA section
;     int_age UINT8
; ends

; fn_hello_world:
;     POP R6  ;; Save return address

;     ;; Allocate stack frame
;     MOV R0, R0(0, -9)  ;; string_hello UINT64 + int_age UINT8

;     PUSH R6  ;; Put the return address back
;     RET



efi_main:
    ;; First order of business, store the pointer to the system table
    MOVREL    R1, system_table  ;; Move system_table into R1
    MOVn      @R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)

    MOVREL R1, string_hello_world
    PUSH R1
    CALL print
    POP R1

    loop_forever:
        JMP loop_forever

    RET
