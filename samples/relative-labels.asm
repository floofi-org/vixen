        jsr foo
        int

foo:
        ldx #12                 ; Set X to 12
        ldy #34                 ; Set Y to 34
        add X, Y                ; 12 + 34
        bne bar                 ; Immediately return
        lda $1000               ; Load from 0x1000
        mov A, R6               ; Move to R6

bar:
        ret