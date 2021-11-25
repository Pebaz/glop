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

    MOVRELq R1, const_u64_2
    PUSH64 R1
    MOVRELq R1, const_u64_3
    PUSH64 R1
    MOVRELq R1, const_u64_4
    PUSH64 R1

    ; MOVI R1, 255
    ; PUSH R1
    ; PUSH R1
    ; PUSH R1
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
    temporary_var_storage: dq ?

;; This is for initialized global variables
section 'DATA' data readable writeable
    const_u64_0: dq 64
    const_u64_1: dq 64
    const_u64_2: dq 255
    const_u64_3: dq 200
    const_u64_4: dq 0
