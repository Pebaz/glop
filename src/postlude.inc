

    ;; END OWN INSTRUCTIONS

    loop_forever:
        JMP loop_forever
    RET

section 'RESERVED' data readable writeable
    system_table: dq ?
    graphics_color: rb EFI_GRAPHICS_OUTPUT_BLT_PIXEL.__size
    efi_graphics_protocol_guid:
        EFI_GUID {0x9042a9de, 0x23dc, 0x4a38, {0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a}}
    graphics_output_protocol: dq ?
    fn_return_storage_u64: dq ?
    literal_0: dq 0  ;; I don't see any other way to make this work
    literal_1: dq 1  ;; I don't see any other way to make this work
    temporary_string_status: du "<HERE>", 0x0D, 0x0A, 0x00

;; This is for initialized global variables
section 'DATA' data readable writeable
