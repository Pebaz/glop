include 'ebc.inc'
include 'efi.inc'
include 'format.inc'
include 'utf8.inc'

format peebc efi  ; PE executable format, EFI Byte Code

entry efi_main

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



;; TODO(pbz): Don't use STACKPUSH & STACKPOP

;; Pushes a 64-bit value onto the stack
;; STACKPUSH(u64)
STACKPUSH:
    POP64 R6  ;; Save return address!

    ;; This can be commented out for the same effect
    POP64 R1  ;; Arg0
    PUSH64 R1  ;; Arg0

    JMP R6  ;; Jump to return address!


;; STACKPOP()
STACKPOP:
    POP64 R6  ;; Save return address!

    POP64 R1

    JMP R6  ;; Jump to return address!



test_func:
    ; POP R6

    MOVREL R1, string_1
    PUSH R1
    CALL print
    POP R1

    JMP32 R6


efi_main:
    ;; First order of business, store the pointer to the system table
    MOVREL    R1, system_table  ;; Move system_table into R1
    MOVn      @R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)


        MOVI R1, 6
        STORESP R6, [IP]

        ;; Add bytes to the address to skip the next instruction
        ADD R6, R1

        JMP test_func  ;; Performing a jump messes up the instruction pointer

        ;; Need to continue from here somehow

        ;; THIS WORKED!
        STORESP R6, [IP]
        PUSH R6
        MOVREL R1, string_2
        PUSH R1
        CALL print
        POP R1
        POP R6

        looop:
            JMP looop


        JMP32 R6




    MOVREL R1, string_hello_world

    ; PUSH R1
    ;; CALL STACKPUSH
    PUSH64 R1
    STORESP R6, [IP]
    MOVi R5, 8
    ADD R6, R5
    MOVREL R5, STACKPUSH
    JMP32 R5

    CALL print
    POP R1

    loop_forever:
        MOVREL R1, string_status
        PUSH R1
        CALL print
        POP R1
        JMP loop_forever

    RET

;; This is for uninitialized global variables and is used in leu of malloc
section 'RESERVED' data readable writeable
    system_table: dq ?

;; This is for initialized global variables
section 'DATA' data readable writeable
    string_hello_world: du "Hello World!", 0x0D, 0x0A, 0x00
    string_status: du "HERE", 0x0D, 0x0A, 0x00
    string_1: du "1", 0x0D, 0x0A, 0x00
    string_2: du "2", 0x0D, 0x0A, 0x00
