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

    STORESP R6, [IP]
    JMP CLEARSCREEN

    MOVREL R1, const_u64_0
    PUSH R1
    MOVREL R1, const_u64_1
    PUSH R1
    MOVREL R1, const_u64_2
    PUSH R1
    MOVREL R1, const_u64_3
    PUSH R1
    MOVREL R1, const_u64_4
    PUSH R1
    STORESP R6, [IP]
    JMP DRAWPIXEL

    MOVREL R1, const_u64_5
    PUSH R1
    MOVREL R1, const_u64_6
    PUSH R1
    MOVREL R1, const_u64_7
    PUSH R1
    MOVREL R1, const_u64_8
    PUSH R1
    MOVREL R1, const_u64_9
    PUSH R1
    STORESP R6, [IP]
    JMP DRAWPIXEL

    MOVREL R1, const_u64_10
    PUSH R1
    MOVREL R1, const_u64_11
    PUSH R1
    MOVREL R1, const_u64_12
    PUSH R1
    MOVREL R1, const_u64_13
    PUSH R1
    MOVREL R1, const_u64_14
    PUSH R1
    STORESP R6, [IP]
    JMP DRAWPIXEL

    MOVREL R1, const_u64_15
    PUSH R1
    MOVREL R1, const_u64_16
    PUSH R1
    MOVREL R1, const_u64_17
    PUSH R1
    MOVREL R1, const_u64_18
    PUSH R1
    MOVREL R1, const_u64_19
    PUSH R1
    STORESP R6, [IP]
    JMP DRAWPIXEL



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
    temporary_string_status: du "<HERE>", 0x0D, 0x0A, 0x00

;; This is for initialized global variables
section 'DATA' data readable writeable
    const_u64_0: dq 64
    const_u64_1: dq 64
    const_u64_2: dq 255
    const_u64_3: dq 200
    const_u64_4: dq 55
    const_u64_5: dq 64
    const_u64_6: dq 128
    const_u64_7: dq 255
    const_u64_8: dq 200
    const_u64_9: dq 55
    const_u64_10: dq 32
    const_u64_11: dq 64
    const_u64_12: dq 255
    const_u64_13: dq 200
    const_u64_14: dq 55
    const_u64_15: dq 32
    const_u64_16: dq 128
    const_u64_17: dq 55
    const_u64_18: dq 200
    const_u64_19: dq 200
