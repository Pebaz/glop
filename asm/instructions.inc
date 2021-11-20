
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

__clear_screen:
    MOVREL    R1, system_table
    MOV       R1, @R1
    MOVn      R1, @R1(EFI_SYSTEM_TABLE.ConOut)
    MOVI   R2, FALSE
    PUSHn  R2
    PUSHn  R1
    CALLEX @R1(SIMPLE_TEXT_OUTPUT_INTERFACE.Reset)
    POPn   R1
    POPn   R2
    RET

;; Pops pointer to string off the stack and prints the string it points to.
EMITSTR:
    CALL __print
    POP R1
    JMP32 R6(+0, +2)

;; Clears the screen. Does not modify the stack.
CLEARSCREEN:
    CALL __clear_screen
    JMP32 R6(+0, +2)


DRAWPIXEL:
    JMP32 R6(+0, +2)
