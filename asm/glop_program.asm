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


;; 1. Pass in digit to convert on stack.
;; 0. Pass in address of return value.
;; Caller must ensure that digit is in range 0-9.
;; Undefined behavior if digit is 10-255.
;; Return value is written to argument 0.
;; void digit_to_utf8(char ret, char arg1)
digit_to_utf8:
    POP R7  ;; Save return address

    MOVI @R0(+1, 0), 48  ;; Assume '0' by default: ret = ord('0');
    MOVI R3, 33  ;; '!'

    CMPI64eq @R0(+2, 0), 0
    JMPcs print_0

    CMPI64eq @R0(+2, 0), 1
    JMPcs print_1

    CMPI64eq @R0(+2, 0), 2
    JMPcs print_2

    CMPI64eq @R0(+2, 0), 3
    JMPcs print_3

    CMPI64eq @R0(+2, 0), 4
    JMPcs print_4

    CMPI64eq @R0(+2, 0), 5
    JMPcs print_5

    CMPI64eq @R0(+2, 0), 6
    JMPcs print_6

    CMPI64eq @R0(+2, 0), 7
    JMPcs print_7

    CMPI64eq @R0(+2, 0), 8
    JMPcs print_8

    CMPI64eq @R0(+2, 0), 9
    JMPcs print_9

    JMP return_digit_to_utf8

    print_0:
        MOVIq R3, 48  ; 0
        JMP return_digit_to_utf8

    print_1:
        MOVIq R3, 49  ; 1
        JMP return_digit_to_utf8

    print_2:
        MOVIq R3, 50  ; 2
        JMP return_digit_to_utf8

    print_3:
        MOVIq R3, 51  ; 3
        JMP return_digit_to_utf8

    print_4:
        MOVIq R3, 52  ; 4
        JMP return_digit_to_utf8

    print_5:
        MOVIq R3, 53  ; 5
        JMP return_digit_to_utf8

    print_6:
        MOVIq R3, 54  ; 6
        JMP return_digit_to_utf8

    print_7:
        MOVIq R3, 55  ; 7
        JMP return_digit_to_utf8

    print_8:
        MOVIq R3, 56  ; 8
        JMP return_digit_to_utf8

    print_9:
        MOVIq R3, 57  ; 9
        JMP return_digit_to_utf8

    return_digit_to_utf8:
        MOV @R0(+1, 0), R3
        PUSH R7  ;; Put the return address back
        RET


;; Print out the digit 2
emit_digit:
    POP R6  ;; Save return address

    ;; Arg1:
    MOVq R2, @R0(+1, 0)
    PUSHn R2

    ;; Arg0: Allocate space for the return value
    MOVIq R2, 0
    PUSHn R2

    CALL digit_to_utf8

    POPn R3  ;; R3 now contains the stringified digit
    POPn R1  ;; Throwaway

    ;; -- WORKS --

    MOVREL R2, string_digit
    ; MOVIb @R2, 55  ;; '7'
    MOVb @R2, R3  ;; Write the stringified digit to the string

    PUSHn R2
    CALL print
    POPn R2

    PUSH R6  ;; Put the return address back
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


test_func3:
    MOVREL R1, string_at
    PUSH R1
    CALL print
    POP R1

    JMP32 R6(+0, +2)  ;; Directly add to the address here!


;; Pop 2 u64s off the stack and push their sum.
ADDU64one:
    POP R3
    POP R4
    ADD R3, R4
    PUSH R3

    MOVREL R1, string_1
    PUSH R1
    CALL print
    POP R1

    ; POP R6


    JMP32 R6(+0, +2)

    RET



efi_main:
    ;; First order of business, store the pointer to the system table
    MOVREL    R1, system_table  ;; Move system_table into R1
    MOVn      @R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)


        ;; Only 2 instructions to call a function!
        STORESP R6, [IP]
        JMP test_func3  ;; Performing a jump messes up the instruction pointer
        ;; Continue from here! :D
        STORESP R6, [IP]
        PUSH R6
        MOVREL R1, string_2
        PUSH R1
        CALL print
        POP R1
        POP R6



        ; ;; This works by itself
        ; MOVI R1, 6
        ; STORESP R6, [IP]
        ; ADD R6, R1  ;; Add bytes to the address to skip the next instruction
        ; JMP test_func  ;; Performing a jump messes up the instruction pointer
        ; ;; Continue from here! :D
        ; STORESP R6, [IP]
        ; PUSH R6
        ; MOVREL R1, string_2
        ; PUSH R1
        ; CALL print
        ; POP R1
        ; POP R6


                    ; ;; Call first assembly routine: ADDU64
                    ; MOVI R1, 3        ;; Arg0
                    ; PUSH R1
                    ; MOVI R1, 1        ;; Arg1
                    ; PUSH R1
                    ; STORESP R6, [IP]  ;; RTNCALL
                    ; ; PUSH R6
                    ; JMP ADDU64one
                    ; ; POP R1
                    ; ; POP R1


                    ; ; MOVREL R1, string_status
                    ; ; PUSH R1
                    ; ; CALL print
                    ; ; POP R1
                    ; ; POP R6

                    ; POP R1

        PUSH R0
        PUSH R0
        PUSH R0
        PUSH R0
        PUSH R0
        PUSH R0
        PUSH R0
        PUSH R0

        MOVI R1, 5
        PUSH R1
        MOVI R1, 4
        PUSH R1

        STORESP R6, [IP]  ;; RTNCALL
        ; MOVREL R1, ADDU64one
        ; JMP32 R1
        JMP ADDU64one
        ;; POP R1  ;; Why does this fix it? <--
        ; POP R1
        ; POP R1
        ; POP R1

        CALL emit_digit
        POP R1



        MOVREL R1, string_status
        PUSH R1
        CALL print
        POP R1

        MOVREL R1, string_status
        PUSH R1
        CALL print
        POP R1


        MOVI R1, 3
        PUSH R1
        CALL emit_digit
        POP R1


        MOVREL R1, string_status
        PUSH R1
        CALL print
        POP R1

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
    string_digit: du "©", 0x0D, 0x0A, 0x00

;; This is for initialized global variables
section 'DATA' data readable writeable
    string_hello_world: du "Hello World!", 0x0D, 0x0A, 0x00
    string_status: du "HERE", 0x0D, 0x0A, 0x00
    string_1: du "1", 0x0D, 0x0A, 0x00
    string_2: du "2", 0x0D, 0x0A, 0x00
    string_at: du "@", 0x0D, 0x0A, 0x00
