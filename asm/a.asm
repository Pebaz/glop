include 'ebc.inc'
include 'efi.inc'
include 'format.inc'
include 'utf8.inc'

format peebc efi  ; PE executable format, EFI Byte Code

entry efi_main

;; This is for all the assembly for the whole program
section 'CODE' code executable readable

include 'instructions.inc'

;; Not yet. All code is global in this file.
; fn_entry:
;     JMP32 R6(+0, +2)

efi_main:
    ;; First order of business, store the pointer to the system table
    MOVREL R1, system_table
    MOVn @R1, @R0(EFI_MAIN_PARAMETERS.SystemTable)

    ;; LocateProtocol(&EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID, 0, (void**)&gop);
    MOVREL    R3, system_table
    MOV       R3, @R3
    MOVn      R3, @R3(EFI_SYSTEM_TABLE.BootServices)

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

    ;; BEGIN OWN INSTRUCTIONS

    ;; Initialize Variables
    ASMCALL CLEARSCREEN

    MOVREL R1, const_0
    PUSH64 R1

    POP64 R2
    MOVq R2, @R2  ;; Assign variable to variable
    MOVREL R1, x
    MOVq @R1, R2

    MOVREL R1, const_1
    PUSH64 R1

    POP64 R2
    MOVq R2, @R2  ;; Assign variable to variable
    MOVREL R1, y
    MOVq @R1, R2

    MOVREL R1, const_2
    PUSH64 R1

    MOVREL R1, const_2
    PUSH64 R1

    MOVREL R1, const_3
    PUSH64 R1

    MOVREL R1, const_4
    PUSH64 R1

    MOVREL R1, const_0
    PUSH64 R1

    ASMCALL DRAWPIXEL

    MOVREL R1, const_1
    PUSH64 R1

    MOVREL R1, const_2
    PUSH64 R1

    MOVREL R1, const_3
    PUSH64 R1

    MOVREL R1, const_4
    PUSH64 R1

    MOVREL R1, const_0
    PUSH64 R1

    ASMCALL DRAWPIXEL

    MOVREL R1, const_5
    PUSH64 R1

    MOVREL R1, const_2
    PUSH64 R1

    MOVREL R1, const_3
    PUSH64 R1

    MOVREL R1, const_4
    PUSH64 R1

    MOVREL R1, const_0
    PUSH64 R1

    ASMCALL DRAWPIXEL

    MOVREL R1, const_2
    PUSH64 R1

    MOVREL R1, const_2
    PUSH64 R1

    MOVREL R1, const_3
    PUSH64 R1

    MOVREL R1, const_4
    PUSH64 R1

    MOVREL R1, const_0
    PUSH64 R1

    ASMCALL DRAWPIXEL

    MOVREL R1, const_2
    PUSH64 R1

    MOVREL R1, const_1
    PUSH64 R1

    MOVREL R1, const_3
    PUSH64 R1

    MOVREL R1, const_4
    PUSH64 R1

    MOVREL R1, const_0
    PUSH64 R1

    ASMCALL DRAWPIXEL

    MOVREL R1, const_2
    PUSH64 R1

    MOVREL R1, const_5
    PUSH64 R1

    MOVREL R1, const_3
    PUSH64 R1

    MOVREL R1, const_4
    PUSH64 R1

    MOVREL R1, const_0
    PUSH64 R1

    ASMCALL DRAWPIXEL

loop_0:

    MOVREL R1, y
    PUSH64 R1

    MOVREL R1, const_5
    PUSH64 R1

    ASMCALL U64GTE

if_0:  ;; UNUSED LABEL
    POP64 R1
    MOVREL R4, literal_1
    CMP64eq R1, @R4
    MOVREL R1, if_0_truthy
    JMP32cs R1
    MOVREL R1, if_0_falsey
    JMP32cc R1
if_0_truthy:
    MOVREL R1, const_6
    PUSH64 R1

    MOVREL R1, const_6
    PUSH64 R1

    MOVREL R1, const_4
    PUSH64 R1

    MOVREL R1, const_0
    PUSH64 R1

    MOVREL R1, const_3
    PUSH64 R1

    ASMCALL DRAWPIXEL

    JMP32 R0(loop_0_break)

    JMP32 R0(if_0_end)
if_0_falsey:
loop_1:

    MOVREL R1, x
    PUSH64 R1

    MOVREL R1, const_1
    PUSH64 R1

    ASMCALL U64GTE

if_1:  ;; UNUSED LABEL
    POP64 R1
    MOVREL R4, literal_1
    CMP64eq R1, @R4
    MOVREL R1, if_1_truthy
    JMP32cs R1
    MOVREL R1, if_1_falsey
    JMP32cc R1
if_1_truthy:
    MOVREL R1, const_7
    PUSH64 R1

    MOVREL R1, const_7
    PUSH64 R1

    MOVREL R1, const_4
    PUSH64 R1

    MOVREL R1, const_0
    PUSH64 R1

    MOVREL R1, const_3
    PUSH64 R1

    ASMCALL DRAWPIXEL

    MOVREL R1, const_0
    PUSH64 R1

    POP64 R2
    MOVq R2, @R2  ;; Assign variable to variable
    MOVREL R1, x
    MOVq @R1, R2

    JMP32 R0(loop_1_break)

    JMP32 R0(if_1_end)
if_1_falsey:
    MOVREL R1, x
    PUSH64 R1

    MOVREL R1, y
    PUSH64 R1

    MOVREL R1, const_4
    PUSH64 R1

    MOVREL R1, const_4
    PUSH64 R1

    MOVREL R1, const_4
    PUSH64 R1

    ASMCALL DRAWPIXEL

    JMP32 R0(if_1_end)
if_1_end: PASS

    MOVREL R1, x
    PUSH64 R1

    MOVREL R1, const_2
    PUSH64 R1

    ASMCALL U64ADD

    POP64 R2
    MOVREL R1, x
    MOVq @R1, R2

    JMP32 R0(loop_1)
loop_1_break: PASS

    JMP32 R0(if_0_end)
if_0_end: PASS

    MOVREL R1, y
    PUSH64 R1

    MOVREL R1, const_2
    PUSH64 R1

    ASMCALL U64ADD

    POP64 R2
    MOVREL R1, y
    MOVq @R1, R2

    JMP32 R0(loop_0)
loop_0_break: PASS



    ;; END OWN INSTRUCTIONS

    loop_forever:
        JMP loop_forever
    RET

;; TODO(pbz): This is temporary. Will want to control this from compiler
;; This is for uninitialized global variables and is used in leu of malloc
section 'RESERVED' data readable writeable
    system_table: dq ?
    graphics_color: rb EFI_GRAPHICS_OUTPUT_BLT_PIXEL.__size
    efi_graphics_protocol_guid:
        EFI_GUID {0x9042a9de, 0x23dc, 0x4a38, {0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a}}
    graphics_output_protocol: dq ?
    literal_0: dq 0  ;; I don't see any other way to make this work
    literal_1: dq 1  ;; I don't see any other way to make this work
    temporary_string_status: du "<HERE>", 0x0D, 0x0A, 0x00

;; This is for initialized global variables
section 'DATA' data readable writeable
    ;; Variables
    x: rb 8
    y: rb 8

    ;; Constants
    const_2: dq 1
    const_1: dq 64
    const_5: dq 128
    const_6: dq 8
    const_7: dq 10
    const_3: dq 55
    const_4: dq 255
    const_0: dq 0
