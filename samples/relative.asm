        jsr     foo
        int

foo:
        mov     R1, #12         ; Set R1 to 12
        mov     R2, #34         ; Set R2 to 34
        add     R1, R2          ; 12 + 34
        bne     #2              ; Immediately return
        ldr     $1000           ; Load from 0x1000
        mov     R6, R0          ; Move to R6
        ret
