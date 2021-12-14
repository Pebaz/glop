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

macro PASS
    OR64 R0, R0
end macro

macro FETCHU64 address
    MOVREL R1, address
    MOVq R1, @R1
    PUSH64 R1
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
CMPU64EQ:
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

    PUSHADDR string_status
    ASMCALL EMITSTR

    ASMCALL LOOPCONTINUE


block_4:
    ;; Need to have better ways of examining the condition
    XOR R1, R1
    PUSH64 R1

    ; FETCHU64 var_x
    ; FETCHU64 var_x
    ; ASMCALL CMPU64EQ

    PUSHADDR block_6  ;; Falsey block
    PUSHADDR block_5  ;; Truthy block
    ASMCALL BRANCHIF

    PUSHADDR string_status
    ASMCALL EMITSTR

    ASMCALL LOOPCONTINUE


;; This is incremented upon IF only, since BREAK only applies to LOOP
block_5_scope_depth: dw 1  ;; Store it right in the binary? =D
block_5:
    PUSHADDR string_b
    ASMCALL EMITSTR

    ; SCOPECLEANUP block_5_scope_depth
    POPn R6  ;; Have to exit the if statement to preserve stack correctly.
             ;; This would be compounded per if statement encountered. When
             ;; generating code, make sure to keep track of this invariant.
             ;; This will most likely be the case for ret also.
             ;; Is this a problem since this block should know it's inside an
             ;; if statement anyway?
    ASMCALL LOOPBREAK

    POPn R6
    JMP32 R6


block_6:
    PUSHADDR string_c
    ASMCALL EMITSTR

    POPn R6
    JMP32 R6



;; u64-gte(a, b) -> return a >= b
U64GTE:
    ; POPn R2  ;; b
    ; POPn R1  ;; a
    ; MOVREL R1, const_0
    ; PUSH64 @R1
    ; JMP32 R6(0, +6)

    POPn R2  ;; b
    POPn R1  ;; a
    CMP64ugte @R1, @R2
    MOVREL R3, U64GTE_truthy
    MOVREL R4, U64GTE_falsey
    JMP32cs R3
    JMP32cc R4

    U64GTE_truthy:
        MOVREL R1, const_1
        PUSH64 @R1
        JMP32 R6(0, +6)

    U64GTE_falsey:
        XOR R1, R1
        PUSH64 R1
        JMP32 R6(0, +6)


efi_main:
    ;; First order of business, store the pointer to the system table
    MOVREL R1, system_table
    MOVn @R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)

    ;; GOAL: Assign to Variable
        PUSHADDR const_u64_0
        PUSHADDR var_x
        ASMCALL ASSIGNU64

        PUSHADDR const_u64_1
        PUSHADDR var_end
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

    ;; GOAL: If v2

        ;; PUSH A u64 VALUE ONTO THE STACK. 0 = false, 1 = true
        ;; (<THE CONDITION EXPRESSION CALL HERE>)
        ;; u64-gte(x, 0)  <- X truly is greater than or equal to 0
        PUSHADDR var_x
        PUSHADDR var_end
        ASMCALL U64GTE

        POP64 R1
        CMPI64eq R1, 0
        MOVREL R1, if_1_falsey
        JMP32cs R1
        MOVREL R1, if_1_truthy
        JMP32cc R1
        if_1_truthy:
            PUSHADDR string_if_1_truthy
            ASMCALL EMITSTR
            JMP32 R0(if_1_endif)
        if_1_falsey:
            PUSHADDR string_if_1_falsey
            ASMCALL EMITSTR
            JMP32 R0(if_1_endif)
        if_1_endif: PASS

    ;; GOAL: Loop
        PUSHADDR block_4
        ASMCALL LOOPBLOCK
        ; STORESP R6, [IP]  ;; Store beginning of block
        ; PUSHn R6(0, +6)

        ; ; POPn R6
        ; ; JMP32 R0, R6

        ; ; CMPeq64 R1, R2

        ; loop_a:  ;; loop.a

        ;     PUSHADDR var_x
        ;     PUSHADDR var_x
        ;     ASMCALL CMPU64EQ

        ;     POP64 R1
        ;     MOViq R1, 1
        ;     CMP64eq R1, R1
        ;     JMPcs loop_a

    ;; GOAL: Loop v2
    ;; Need to turn loop [] into something
        loop_1:  ;; Special blocks start with their name?
            loop_2:  ;; Special blocks start with their name?
                JMP32 R0(loop_2_break)  ;; Break the loop

                JMP32 R0(loop_2)
            loop_2_break: PASS

            JMP32 R0(loop_1_break)  ;; Break the loop

            JMP32 R0(loop_1)
        loop_1_break: PASS

    PUSHADDR string_done
    ASMCALL EMITSTR

    loop_forever:
        JMP loop_forever
    RET

;; TODO(pbz): This is temporary. Will want to control this from compiler
;; This is for uninitialized global variables and is used in leu of malloc
section 'RESERVED' data readable writeable
    system_table: dq ?
    const_0: dq 0  ;; I don't see any other way to make this work
    const_1: dq 1  ;; I don't see any other way to make this work
    string_done: du "<DONE>", 0x0D, 0x0A, 0x00
    string_status: du "<HERE>", 0x0D, 0x0A, 0x00
    string_true: du "<true>", 0x0D, 0x0A, 0x00
    string_false: du "<false>", 0x0D, 0x0A, 0x00
    string_a: du "<a>", 0x0D, 0x0A, 0x00
    string_b: du "<b>", 0x0D, 0x0A, 0x00
    string_c: du "<c>", 0x0D, 0x0A, 0x00
    string_if_1_truthy: du "<if_1_truthy>", 0x0D, 0x0A, 0x00
    string_if_1_falsey: du "<if_1_falsey>", 0x0D, 0x0A, 0x00

    ;; GOAL: Declare Variable
        var_x: rb 8
        var_end: rb 8

;; This is for initialized global variables
section 'DATA' data readable writeable
    const_u64_0: dq 64
    const_u64_1: dq 0
