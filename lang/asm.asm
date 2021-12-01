include 'ebc.inc'
include 'efi.inc'
include 'format.inc'
include 'utf8.inc'

format peebc efi  ; PE executable format, EFI Byte Code

entry efi_main

;; This is for all the assembly for the whole program
section 'CODE' code executable readable

macro ASMCALL routine_name
    STORESP R6, [IP]
    JMP32 R0(routine_name)
end macro

;; Uses register R1 and pushes a natural value to account for x32 and x64.
macro PUSHADDR var_name
    MOVREL R1, var_name
    PUSHn R1
end macro

EMITSTR:
    POPn R1
    PUSHn R6(0, +6)

    PUSHn R1
    CALL __print
    POPn R1

    POPn R6
    JMP32 R6

__print:
    MOVREL    R1, system_table
    MOV       R1, @R1
    MOVn      R1, @R1(EFI_SYSTEM_TABLE.ConOut)
    PUSHn     @R0(0,+16)
    PUSHn     R1
    ; CALLEX    @R1(SIMPLE_TEXT_OUTPUT_INTERFACE.OutputString)
    MOV       R1, @R1(SIMPLE_TEXT_OUTPUT_INTERFACE.OutputString)
    CALLEX    R1
    MOV       R0, R0(+2,0)
    RET

;; Push value to assign first and then variable.
ASSIGNU64:
    POPn R1  ;; variable
    POPn R2  ;; value to assign

    MOVq @R1, @R2

    JMP32 R6(0, +6)

;; Push b first then a. Returns 0 if a == b. Returns 1 if a != b.
COMPAREU64EQ:
    POPn R1  ;; a
    POPn R2  ;; b
    CMP64eq @R1, @R2
    JMPcs equal
    JMPcc not_equal

    equal:
        XOR32 R1, R1  ;; Store zero
        PUSH64 R1

        JMP32 R6(0, +6)

    not_equal:
        MOViq R1, 1
        PUSH64 R1

        JMP32 R6(0, +6)


;; Push condition, push falsey address, push truthy address.
;; Runs truthy block when condition is 0, falsey block otherwise.
BRANCHIF:
    POPn R3  ;; Truthy address
    POPn R2  ;; Falsey address
    POP64 R1  ;; Condition
    PUSHn R6(0, +6)  ;; Store return address

    XOR R4, R4  ;; Get comparitor (0)
    CMP64eq R1, R4

    JMP32cs R3         ;; Both blocks must pop and jump to R6 when done
    JMP32cc R2         ;; Both blocks must pop and jump to R6 when done


;; GOAL: Break -> implement as ret.loop-name?
;; GOAL: Struct
;; GOAL: Function

;; Block index counter is incremented each new block no matter the type
block_2:
    PUSHADDR string_false
    ASMCALL EMITSTR

    POPn R6
    JMP32 R6

block_1:  ;; Basic blocks are named. Need a handle to their address
    PUSHADDR string_true
    ASMCALL EMITSTR

    POPn R6
    JMP32 R6


;; Loops store their break address before looping the block. Doing an extra pop
;; results in not returning back to the LOOPBLOCK instruction, but after it.
LOOPBREAK:  ;; Break.label tells how many times to repeat this instruction?
    ;; Ignore caller return address, we want to go to after the loop
    JMP32 R0(LOOPBLOCK_break)


LOOPCONTINUE:
    ;; Ignore caller return address, we want to go back to LOOPBLOCK's control
    JMP32 R0(LOOPBLOCK_continue)


;; Push address of block to loop forever.
LOOPBLOCK:
    POPn R1  ;; Block address

    ;; Return address to use after breaking the loop is in R6
    PUSHn R6(0, +6)

    ;; Top of the stack is reserved for the block address
    PUSHn R1

    ASMCALL LOOPCONTINUE

    LOOPBLOCK_continue:
        POPn R1  ;; Block address
        PUSHn R1
        JMP32 R1


    LOOPBLOCK_break:
        POPn R6  ;; Get rid of the loop block address
        POPn R6  ;; The address to continue past the loop
        JMP32 R6


block_3:  ;; The block to loop
    ;; How does it know how to break out of the loop?

    PUSHADDR string_status
    ASMCALL EMITSTR

    ASMCALL LOOPCONTINUE


efi_main:
    ;; First order of business, store the pointer to the system table
    MOVREL R1, system_table
    MOVn @R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)

    ;; GOAL: Assign to Variable
        PUSHADDR const_u64_0
        PUSHADDR var_x
        ASMCALL ASSIGNU64

    ;; GOAL: If
        ;; `if @foo() [] else []` is actually `@foo(), if [] else []`

        ;; <CALL SOME FUNCTION HERE THAT RETURNS U64 VALUE ON THE STACK>
        ;; This shall represent the condition
        XOR R1, R1  ;; Push 0 (true for purposes of comparison)
        PUSH64 R1

        ;; Value to compare is already on stack!
        PUSHADDR block_2  ;; Falsey block
        PUSHADDR block_1  ;; Truthy block
        ASMCALL BRANCHIF  ;; Will continue from here after one block is run

    ;; GOAL: Loop
        PUSHADDR block_3
        ASMCALL LOOPBLOCK
        ; STORESP R6, [IP]  ;; Store beginning of block
        ; PUSHn R6(0, +6)

        ; ; POPn R6
        ; ; JMP32 R0, R6

        ; ; CMPeq64 R1, R2

        ; loop_a:  ;; loop.a

        ;     PUSHADDR var_x
        ;     PUSHADDR var_x
        ;     ASMCALL COMPAREU64EQ

        ;     POP64 R1
        ;     MOViq R1, 1
        ;     CMP64eq R1, R1
        ;     JMPcs loop_a

    PUSHADDR string_done
    ASMCALL EMITSTR

    loop_forever:
        JMP loop_forever
    RET

;; TODO(pbz): This is temporary. Will want to control this from compiler
;; This is for uninitialized global variables and is used in leu of malloc
section 'RESERVED' data readable writeable
    system_table: dq ?
    const_1: db 1
    string_done: du "<DONE>", 0x0D, 0x0A, 0x00
    string_status: du "<HERE>", 0x0D, 0x0A, 0x00
    string_true: du "<true>", 0x0D, 0x0A, 0x00
    string_false: du "<false>", 0x0D, 0x0A, 0x00
    string_a: du "<a>", 0x0D, 0x0A, 0x00

    ;; GOAL: Declare Variable
        var_x: rb 8

;; This is for initialized global variables
section 'DATA' data readable writeable
    const_u64_0: dq 64
