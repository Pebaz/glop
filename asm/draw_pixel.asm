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

section '.text' code executable readable

print:
    MOVREL    R1, system_table
    MOV       R1, @R1
    MOVn      R1, @R1(EFI_SYSTEM_TABLE.ConOut)
    PUSHn     @R0(0,+16)
    PUSHn     R1
    CALLEX    @R1(SIMPLE_TEXT_OUTPUT_INTERFACE.OutputString)
    MOV       R0, R0(+2,0)
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


efi_main:
    MOVREL    R1, system_table  ;; Move system_table into R1
    MOVn      @R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)

    CALL clear_screen

    MOVREL    R3, system_table
    MOV       R3, @R3
    MOVn      R3, @R3(EFI_SYSTEM_TABLE.BootServices)

    MOVREL      R5, graphics_color
    MOVibw      @R5(EFI_GRAPHICS_OUTPUT_BLT_PIXEL.Red), 255
    MOVibw      @R5(EFI_GRAPHICS_OUTPUT_BLT_PIXEL.Green), 200
    MOVibw      @R5(EFI_GRAPHICS_OUTPUT_BLT_PIXEL.Blue), 0

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

    ;; graphics_output_protocol is now useable!
    ;; gop->Blt(gop, &GraphicsColor, EfiBltVideoFill, 0, 0, 256, 256, 1, 1, 0);
    MOVi R4, 0
    PUSHn R4
    MOVi R4, 1
    PUSHn R4
    MOVi R4, 1
    PUSHn R4
    MOVi R4, 64
    PUSHn R4
    MOVi R4, 64
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

    MOVREL    R1, string_status
    PUSH      R1
    CALL      print
    POP       R1

    MOVREL R3, graphics_output_protocol
    MOV R3, @R3
    CALLEX @R3(EFI_GRAPHICS_OUTPUT_PROTOCOL.Blt)

    POPn      R5
    POPn      R5
    POPn      R5
    POPn      R5
    POPn      R5
    POPn      R5
    POPn      R5
    POPn      R5
    POPn      R5
    POPn      R5

    MOVREL    R1, string_status
    PUSH      R1
    CALL      print
    POP       R1

    JMP loop_forever
    RET

loop_forever:
    JMP loop_forever

;; ! http://flatassembler.net/docs.php?article=fasmg_manual
section '.data' data readable writeable
    ;; Reserve 8 bytes in the .data section to use for storing a pointer
    system_table: dq ?
    efi_graphics_protocol_guid:
        EFI_GUID {0x9042a9de, 0x23dc, 0x4a38, {0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a}}
    graphics_output_protocol: dq ?
    event_wait_for_key: dq ?  ;; I think this stays 0? 0 = WaitForKey?
    string_succeed: du "YES", 0x0A, 0x00
    string_failed: du "NO", 0x0A, 0x00
    string_status: du "HERE", 0x0A, 0x00
    graphics_color: rb EFI_GRAPHICS_OUTPUT_BLT_PIXEL.__size
