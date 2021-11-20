

    ;; END OWN INSTRUCTIONS

    looop:
        JMP looop
    RET

;; TODO(pbz): This is temporary. Will want to control this from compiler
;; This is for uninitialized global variables and is used in leu of malloc
section 'RESERVED' data readable writeable
    system_table: dq ?

