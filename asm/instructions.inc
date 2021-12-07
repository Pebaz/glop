macro ASMCALL routine_name
    STORESP R6, [IP]
    JMP32 R0(routine_name)
end macro

macro PASS
    OR64 R0, R0
end macro

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


;; Reversed parameters due to C calling convention
struct FFI_FN_EFI_GRAPHICS_OUTPUT_PROTOCOL_BLT
    Delta UINT64
    Height UINT64  ;; <- Can already draw rectangle pixels!
    Width UINT64
    DestinationY UINT64
    DestinationX UINT64
    SourceY UINT64
    SourceX UINT64
    BltOperation UINT64
    BltBuffer EFI_GRAPHICS_OUTPUT_BLT_PIXEL
    This UINT64
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
    ;; gop->Blt(gop, &GraphicsColor, EfiBltVideoFill, 0, 0, x, y, 1, 1, 0);

    ; MOVq R3, @R0(+1, 0)
    ; MOVq R5, @R0(+2, 0)

    MOVi R4, 0
    PUSH64 R4
    MOVi R4, 1
    PUSH64 R4
    MOVi R4, 1
    PUSH64 R4
    MOVn R4, @R0(+5, 0)  ;; +3 previous PUSHes
    PUSH64 R4
    MOVn R4, @R0(+7, 0)  ;; +4 previous PUSHes
    PUSH64 R4
    MOVi R4, 0
    PUSH64 R4
    MOVi R4, 0
    PUSH64 R4
    MOVi R4, EfiBltVideoFill
    PUSH64 R4
    MOVREL R2, graphics_color  ;; This is a pointer to a struct
    PUSH64 R2
    MOVREL R2, graphics_output_protocol  ;; This is a pointer to a pointer
    MOV R2, @R2
    PUSH64 R2

    MOVREL R3, graphics_output_protocol
    MOV R3, @R3
    CALLEX @R3(EFI_GRAPHICS_OUTPUT_PROTOCOL.Blt)

    MOV R0, R0(+10, 0)

    RET


;; Clears the screen. Does not modify the stack.
CLEARSCREEN:
    PUSHn R6(0, +6)

    CALL __clear_screen

    POPn R6
    JMP32 R6


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
    POP64 R3
    POP64 R4

    PUSHn R6(0, +6)

    MOVREL R2, 64
    PUSH64 @R4
    PUSH64 @R3
    CALL __draw_pixel
    POP64 R1
    POP64 R1

    ; MOV R1, R0
    ; MOV R0, R0(0, -sizeof FFI_FN_EFI_GRAPHICS_OUTPUT_PROTOCOL_BLT)

    ; MOVREL @R1(FFI_FN_EFI_GRAPHICS_OUTPUT_PROTOCOL_BLT.This), graphics_output_protocol
    ; MOVREL R2, graphics_color
    ; MOV @R1(FFI_FN_EFI_GRAPHICS_OUTPUT_PROTOCOL_BLT.BltBuffer), R2
    ; MOVi R2, EfiBltVideoFill
    ; MOV @R1(FFI_FN_EFI_GRAPHICS_OUTPUT_PROTOCOL_BLT.BltOperation), R2
    ; MOVi R2, 0
    ; MOV @R1(FFI_FN_EFI_GRAPHICS_OUTPUT_PROTOCOL_BLT.SourceX), R2
    ; MOV @R1(FFI_FN_EFI_GRAPHICS_OUTPUT_PROTOCOL_BLT.SourceY), R2
    ; MOV @R1(FFI_FN_EFI_GRAPHICS_OUTPUT_PROTOCOL_BLT.DestinationX), @R3
    ; MOV @R1(FFI_FN_EFI_GRAPHICS_OUTPUT_PROTOCOL_BLT.DestinationY), @R4
    ; MOVi R2, 1
    ; MOV @R1(FFI_FN_EFI_GRAPHICS_OUTPUT_PROTOCOL_BLT.Width), R2
    ; MOV @R1(FFI_FN_EFI_GRAPHICS_OUTPUT_PROTOCOL_BLT.Height), R2
    ; MOVi R2, 0
    ; MOV @R1(FFI_FN_EFI_GRAPHICS_OUTPUT_PROTOCOL_BLT.Delta), R2

    ; MOVREL R3, graphics_output_protocol
    ; MOV R3, @R3
    ; CALLEX @R3(EFI_GRAPHICS_OUTPUT_PROTOCOL.Blt)

    POPn R6
    ; MOV R0, R0(0, sizeof FFI_FN_EFI_GRAPHICS_OUTPUT_PROTOCOL_BLT)


    JMP32 R6


;; Pops pointer to string off the stack and prints the string it points to.
EMITSTR:
    POP64 R2  ;; Arg0
    PUSHn R6(0, +6)

    MOVREL    R1, system_table
    MOV       R1, @R1
    MOVn      R1, @R1(EFI_SYSTEM_TABLE.ConOut)

    PUSH64 R2  ;; Arg0
    PUSH64 R1

    CALLEX    @R1(SIMPLE_TEXT_OUTPUT_INTERFACE.OutputString)

    POP64 R1
    POP64 R1  ;; Arg0

    POPn R6
    JMP32 R6


;; Adds two numbers and pushes the result onto the stack.
U64ADD:
    POP64 R2  ;; Arg1
    POP64 R1  ;; Arg0

    MOVq R1, @R1
    MOVq R2, @R2

    ADD64 R1, R2
    PUSH64 R1

    JMP32 R6(0, +6)


;; u64-gte(a, b) -> return a >= b
U64GTE:
            ; POPn R2  ;; b
            ; POPn R1  ;; a
            ; MOVREL R1, literal_0
            ; PUSH64 @R1
            ; JMP32 R6(0, +6)

    POP64 R2  ;; b
    POP64 R1  ;; a
    MOVq R1, @R1
    MOVq R2, @R2
    CMP64ugte R1, R2
    MOVREL R3, U64GTE_truthy
    MOVREL R4, U64GTE_falsey
    JMP32cs R3
    JMP32cc R4

    U64GTE_truthy:
        MOVRELq R1, literal_1
        PUSH64 @R1
        JMP32 R6(0, +6)

    U64GTE_falsey:
        XOR R1, R1
        PUSH64 R1
        JMP32 R6(0, +6)


;; u64-gte(a, b) -> return a >= b
U64EQ:
            ; POPn R2  ;; b
            ; POPn R1  ;; a
            ; MOVREL R1, literal_1
            ; PUSH64 @R1
            ; JMP32 R6(0, +6)

    POP64 R2  ;; b
    POP64 R1  ;; a
    MOVq R1, @R1
    MOVq R2, @R2
    CMP64eq R1, R2

    MOVREL R1, U64EQ_truthy
    JMP32cs R1
    MOVREL R1, U64EQ_falsey
    JMP32cc R1

    U64EQ_truthy:
        MOVREL R1, literal_1
        PUSH64 @R1
        JMP32 R6(0, +6)

    U64EQ_falsey:
        ; XOR R1, R1
        ; PUSH64 R1
        MOVREL R1, literal_0
        PUSH64 @R1
        JMP32 R6(0, +6)
