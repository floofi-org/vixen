        jsr     foo
        int

foo:
        mov     r1, #12         ; Set R1 to 12
        mov     r2, #34         ; Set R2 to 34
        add     r1, r1, r2      ; 12 + 34
        bne     bar             ; Immediately return
        mov     r0, $1000       ; Load from 0x1000
        mov     r6, r0          ; Move to R6

bar:
        ret
