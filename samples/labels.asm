        jsr     #foo
        jsr     #bar
        int

foo:
        mov     r1, #12         ; Set r1 to 12
        mov     r2, #34         ; Set r2 to 34
        add     r1, r1, r2      ; 12 + 34
        mov     r1, $1000       ; Store at 0x1000
        ret

bar:
        mov     r0, $1000       ; Load from 0x1000
        mov     r6, r0          ; Move to r6
        ret
