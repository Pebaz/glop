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
    PUSH R6  ;; Save IP

    MOVREL R1, string_1
    PUSH R1
    CALL print
    POP R1

    POP R6  ;; Retrieve IP
    JMP32 R6



; ;; TODO ----------> perhaps push return address last?
; test_func2:
;     ;; Args
;     PUSH arg0
;     PUSH arg1
;     PUSH arg2

;     ;; Call
;     MOVI R1, 6
;     STORESP R6, [IP]
;     ADD R6, R1
;     PUSH R6  ;; NEED TO ACCOUNT FOR THIS
;     JMP the_function_to_call


; the_function_to_call:



;; CAN ALL OF THESE BE ASM ROUTINES?
;; JMP SYSCALL <- Wow. Just wow.
;; JMP FNCALL <- Nice, but could just jump directly to label?
;; JMP ASMCALL <- Cool, but again could just jump directly
;; The sick part is that all 3 of those are really assembly routines!

;; Handles all the register business for you but still acts like an ASMCALL
;; Arguments must be popped back off manually
;; C calling convention
;; EBC defined
SYSCALL:
    db 0

;; A normal function call, has a return value
;; Manipulates the stack to get arguments
;; Glop source code
FNCALL:
    db 0


;; PERHAPS ASMCALL SHOULDN'T EXIST. JUST INLINE EVERYTHING.
;; An assembly routine, only works with the logical stack
;; Manipulates the stack to get arguments
;; Handmade for each platform
;; ASMCALL:
    ;; NUM ARGS
    ;; JUMP TO ADDRESS




efi_main:
    ;; First order of business, store the pointer to the system table
    MOVREL    R1, system_table  ;; Move system_table into R1
    MOVn      @R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)


        ;; This works by itself
        MOVI R1, 6
        STORESP R6, [IP]
        ADD R6, R1  ;; Add bytes to the address to skip the next instruction
        JMP test_func  ;; Performing a jump messes up the instruction pointer
        ;; Continue from here! :D
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
