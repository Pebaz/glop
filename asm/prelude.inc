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

