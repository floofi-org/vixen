        jsr     foo
        jsr     bar
        int

foo:
        ldx     #12             ; Set X to 12
        ldy     #34             ; Set Y to 34
        add     X, Y            ; 12 + 34
        sta     $1000           ; Store at 0x1000
        ret

bar:    lda     $1000           ; Load from 0x1000
        mov     A, R6           ; Move to R6
        ret
