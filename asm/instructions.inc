struct EFI_GRAPHICS_OUTPUT_PROTOCOL
    QueryMode   VOID_PTR
    SetMode     VOID_PTR
    Blt         VOID_PTR
    Mode        VOID_PTR
ends

struct EFI_GRAPHICS_OUTPUT_BLT_PIXEL
    Blue        UINT8
    Green       UINT8
    Red         UINT8
    Reserved    UINT8
ends

EfiBltVideoFill = 0

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

;; void __draw_pixel(char x, char y);
__draw_pixel:
    ;; graphics_output_protocol is now useable!
    ;; gop->Blt(gop, &GraphicsColor, EfiBltVideoFill, 0, 0, 256, 256, 1, 1, 0);

    MOVi R4, 0
    PUSHn R4
    MOVi R4, 1
    PUSHn R4
    MOVi R4, 1
    PUSHn R4
    MOV R4, @R0(+5, 0)  ;; +3 previous PUSHes
    PUSHn R4
    MOV R4, @R0(+7, 0)  ;; +4 previous PUSHes
    PUSHn R4
    MOVi R4, 0
    PUSHn R4
    MOVi R4, 0
    PUSHn R4
    MOVi R4, EfiBltVideoFill
    PUSHn R4
    MOVREL R2, graphics_color  ;; This is a pointer to a struct
    PUSHn R2
    MOVREL R2, graphics_output_protocol  ;; This is a pointer to a pointer
    MOV R2, @R2
    PUSHn R2

    MOVREL R3, graphics_output_protocol
    MOV R3, @R3
    CALLEX @R3(EFI_GRAPHICS_OUTPUT_PROTOCOL.Blt)

    MOV R0, R0(+10, 0)

    RET

;; void set_pen_color(char r, char g, char b);
;; Arguments should be pushed on stack.
;; Uses registers R5-R6.
__set_pen_color:



    ; ::::::::::::::::::
    ; Is the return address 16 bit? Why did this work?


    ; POP R7  ;; Save return address
    MOVREL R5, graphics_color

    MOVb @R5(EFI_GRAPHICS_OUTPUT_BLT_PIXEL.Blue), @R0(+1, 16)
    MOVb @R5(EFI_GRAPHICS_OUTPUT_BLT_PIXEL.Green), @R0(+2, 16)
    MOVb @R5(EFI_GRAPHICS_OUTPUT_BLT_PIXEL.Red), @R0(+3, 16)

    ; PUSH R7  ;; Put the return address back
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


;; Caller is pushing references to the variable's memory, not by value.
DRAWPIXEL:

    ;; Fetch memory pointed to by arg2, arg3, and arg4
    MOVREL R5, graphics_color
    POP64 R1
    MOVb @R5(EFI_GRAPHICS_OUTPUT_BLT_PIXEL.Blue), @R1
    POP64 R1
    MOVb @R5(EFI_GRAPHICS_OUTPUT_BLT_PIXEL.Green), @R1
    POP64 R1
    MOVb @R5(EFI_GRAPHICS_OUTPUT_BLT_PIXEL.Red), @R1

    ;; Fetch memory pointed to by arg0 and arg1
    POP64 R1
    POP64 R2
    PUSH64 @R1
    PUSH64 @R2

    CALL __draw_pixel
    POPn R1
    POPn R1

    JMP32 R6(+0, +2)


print:
    MOVREL    R1, system_table
    MOV       R1, @R1
    MOVn      R1, @R1(EFI_SYSTEM_TABLE.ConOut)
    PUSHn     @R0(0,+16)
    PUSHn     R1
    CALLEX    @R1(SIMPLE_TEXT_OUTPUT_INTERFACE.OutputString)
    MOV       R0, R0(+2,0)
    RET
