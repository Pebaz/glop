include 'ebc.inc'
include 'efi.inc'
include 'format.inc'
include 'utf8.inc'

format peebc efi  ; PE executable format, EFI Byte Code

entry efi_main

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

;; .text
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
    ; ;; Arg1 by value:
    ; MOVI R1, 0
    ; PUSHN R1

    ; ;; Arg0 (ret) by reference:
    ; MOVI R1, 0
    ; PUSHN R1

    ;; ret = ord('0'); while (arg1) { arg1 -= 1; ret += 1 } return ret;


    ;; WORKS, USE MOV to overwrite entire register. That's how it works!
    ; MOVIb R1, 56
    ; MOVq @R0(+2, 0), R1
    ; RET

    MOVI @R0(+2, 0), 48  ;; Assume '0' by default: ret = ord('0');

    ;; Upper: 125000000
    ;; Lower: 124000000
    ;; Lower: 124000000
    CMPIgte @R0(+1, 0), 124000000
    JMPcs yes
    JMPcc no

    yes:
        MOVREL R3, string_higher
        PUSH R3
        CALL print
        POP R3
        RET
    no:
        MOVREL R3, string_lower
        PUSH R3
        CALL print
        POP R3
        RET

    RET

    MOVI R3, 33

    CMPI64eq @R0(+1, 0), 0
    JMPcs print_0

    CMPI64eq @R0(+1, 0), 1
    JMPcs print_1

    CMPI64eq @R0(+1, 0), 2
    JMPcs print_2

    CMPI64eq @R0(+1, 0), 3
    JMPcs print_3

    CMPI64eq @R0(+1, 0), 4
    JMPcs print_4

    CMPI64eq @R0(+1, 0), 5
    JMPcs print_5

    CMPI64eq @R0(+1, 0), 6
    JMPcs print_6

    CMPI64eq @R0(+1, 0), 7
    JMPcs print_7

    CMPI64eq @R0(+1, 0), 8
    JMPcs print_8

    CMPI64eq @R0(+1, 0), 9
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
        MOV @R0(+2, 0), R3
        RET

    ; CMPIlte @R0(+1, 0), 0  ;; if (arg1 <= 0)
    ; JMPcs return

    ; count_down:
    ;     MOVREL R5, string_status
    ;     PUSH R5
    ;     CALL print
    ;     POP R5

    ;     ;; arg1 -= 1;
    ;     MOVI R2, 1
    ;     MOV R1, @R0(+1, 0)
    ;     SUB R1, R2
    ;     MOV @R0(+1, 0), R1

    ;     ;; ret += 1;
    ;     MOVI R2, 1
    ;     MOV R1, @R0(+2, 0)
    ;     ADD R1, R2
    ;     MOV @R0(+2, 0), R1

    ;     CMPIlte @R0(+1, 0), 0  ;; if (arg1 <= 0) break;
    ;     JMPcc count_down  ;; else loop

    ; return:
    ;     RET


;; Print out the digit 2
emit_digit:
    ;; Arg1:
    MOVIq R2, 2
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
    RET


clear_screen:
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


;; void set_pen_color(char r, char g, char b);
;; Arguments should be pushed on stack.
;; Uses registers R5-R6.
set_pen_color:
    POP R7  ;; Save return address
    MOVREL R5, graphics_color

    MOVb @R5(EFI_GRAPHICS_OUTPUT_BLT_PIXEL.Blue), @R0(+1, 0)
    MOVb @R5(EFI_GRAPHICS_OUTPUT_BLT_PIXEL.Green), @R0(+2, 0)
    MOVb @R5(EFI_GRAPHICS_OUTPUT_BLT_PIXEL.Red), @R0(+3, 0)

    PUSH R7  ;; Put the return address back
    RET


;; void draw_pixel(char x, char y)  // , char r, char g, char b);
;; Arguments should be pushed on stack.
draw_pixel:
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




;; char * return_string() { if (1) { return string_success; } else { return string_failed; } }
fn_return_string_working:  ;; ! This is working
    POP R7  ;; Save return address

    ;; ret = string_yes; OR: arg0 = string_yes;
    MOVREL R1, string_yes
    MOVq @R0(+1, 0), R1  ;; Move &string_yes in R1 to STACK[-1]

    PUSH R7  ;; Put the return address back
    RET


fn_return_string:
    POP R7  ;; Save return address

    MOVRELq R1, return_first_string
    MOVRELq R2, return_first_string_comparitor
    ;; CMP64eq R1, R2  ;; THIS IS SIGNED :|  <------------------------------------
    CMP64ulte R1, R2
    CMP64ugte R1, R2
    JMPcs fn_return_string_set_string_yes
    JMPcc fn_return_string_set_string_no

    fn_return_string_set_string_yes:
        MOVREL R1, string_yes
        MOVq @R0(+1, 0), R1  ;; Move &string_yes in R1 to STACK[-1]
        JMP fn_return_string_end_scope

    fn_return_string_set_string_no:
        MOVREL R1, string_no
        MOVq @R0(+1, 0), R1  ;; Move &string_no in R1 to STACK[-1]
        JMP fn_return_string_end_scope

    fn_return_string_end_scope:
        PUSH R7  ;; Put the return address back
        RET


push_registers:
    ;; PUSH R0  <- Won't work with RET
    PUSH R1
    PUSH R2
    PUSH R3
    PUSH R4
    PUSH R5
    PUSH R6
    PUSH R7
    RET

pop_registers:
    POP R7
    POP R6
    POP R5
    POP R4
    POP R3
    POP R2
    POP R1
    ;; POP R0  <- Won't work with RET
    RET

efi_main:
    MOVREL    R1, system_table  ;; Move system_table into R1
    MOVn      @R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)

    CALL clear_screen

    MOVREL    R3, system_table
    MOV       R3, @R3
    MOVn      R3, @R3(EFI_SYSTEM_TABLE.BootServices)

    ; MOVREL      R5, graphics_color
    ; MOVibw      @R5(EFI_GRAPHICS_OUTPUT_BLT_PIXEL.Red), 255
    ; MOVibw      @R5(EFI_GRAPHICS_OUTPUT_BLT_PIXEL.Green), 200
    ; MOVibw      @R5(EFI_GRAPHICS_OUTPUT_BLT_PIXEL.Blue), 0

    MOVi R5, 255
    PUSH64 R5
    MOVi R5, 200
    PUSH64 R5
    MOVi R5, 0
    PUSH64 R5
    CALL set_pen_color

    ;; LocateProtocol(&EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID, 0, (void**)&gop);

    MOVREL    R5, graphics_output_protocol
    PUSHn     R5
    MOVi      R5, 0
    PUSHn     R5
    MOVREL    R5, efi_graphics_protocol_guid
    PUSHn     R5

    CALLEX    @R3(EFI_BOOT_SERVICES.LocateProtocol)

    POPn      R5
    POPn      R5
    POPn      R5

    ; MOVREL R2, event_wait_for_key
    ; PUSHn  R2
    ; MOV    R1, R1(SIMPLE_TEXT_INPUT_INTERFACE.WaitForKey)
    ; PUSHn  R1
    ; MOVI   R1, 1
    ; PUSHn  R1
    ; CALLEX @R3(EFI_BOOT_SERVICES.WaitForEvent)
    ; MOV    R0, R0(+3,0)

    CMPIeq R7, 0
    JMPcc else_block  ;; If not equal and condition cleared (CC), jump

    MOVREL R1, string_succeed  ;; Continue onto truthy block
    JMP continue

else_block:
    MOVREL R1, string_failed  ;; Continue onto truthy block
    JMP continue

continue:
    PUSH      R1
    CALL      print
    POP       R1

    ;; draw_pixel(192, 64);
    MOVi R4, 192
    PUSH R4
    MOVi R4, 64
    PUSH R4
    CALL draw_pixel
    POP R4
    POP R4

    ;; draw_pixel(128, 64);
    MOVi R4, 128
    PUSH R4
    MOVi R4, 64
    PUSH R4
    CALL draw_pixel
    POP R4
    POP R4

    ;; print(string_status);
    MOVREL    R1, string_status
    PUSH      R1
    CALL      print
    POP       R1

            MOVREL    R1, string_status
            PUSH      R1
            CALL      print
            POP       R1

            ;; TODO(pbz): Finish this function
            ;; CALL emit_digit

            MOVREL    R1, string_line
            PUSH      R1
            CALL      print
            POP       R1

            ;; --
            PUSH64 R4  ;; Allocate space for return value
            CALL fn_return_string
            POP64 R4  ;; Deallocate return value

            PUSH64 R4  ;; Push arg
            CALL print
            POP64 R4  ;; Pop arg

            MOVREL    R1, string_line
            PUSH      R1
            CALL      print
            POP       R1

    RET

    ;; for (int i = 0; i < 256; i++) { draw_pixel(i, i); }
    MOVi R1, 1
    draw_line:
        PUSH R1
        PUSH R1
        CALL draw_pixel
        POP R4
        POP R4

        MOVi R2, 1
        ADD R1, R2

        CMPIgte R1, 256
        JMPcc draw_line

    JMP loop_forever
    RET

loop_forever:
    JMP loop_forever

;; ! http://flatassembler.net/docs.php?article=fasmg_manual
;; .data
section 'DATA' data readable writeable
    ;; Reserve 8 bytes in the .data section to use for storing a pointer
    system_table: dq ?
    efi_graphics_protocol_guid:
        EFI_GUID {0x9042a9de, 0x23dc, 0x4a38, {0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a}}
    graphics_output_protocol: dq ?
    event_wait_for_key: dq ?  ;; I think this stays 0? 0 = WaitForKey?
    string_succeed: du "YES", 0x0D, 0x0A, 0x00
    string_failed: du "NO", 0x0D, 0x0A, 0x00
    string_status: du "HERE", 0x0D, 0x0A, 0x00  ;; Windows line endings: \r\n
    graphics_color: rb EFI_GRAPHICS_OUTPUT_BLT_PIXEL.__size
    string_line: du "----------", 0x0D, 0x0A, 0x00

    string_higher: du "^", 0x0D, 0x0A, 0x00  ;; Windows line endings: \r\n
    string_lower: du "v", 0x0D, 0x0A, 0x00  ;; Windows line endings: \r\n

    return_first_string: dq 1
    return_first_string_comparitor: dq 1
    string_yes: du "<YES>", 0x0D, 0x0A, 0x00
    string_no: du "<NO>", 0x0D, 0x0A, 0x00


;; .bss
section 'RESERVED' data readable writeable
    string_digit: du "©", 0x0D, 0x0A, 0x00
