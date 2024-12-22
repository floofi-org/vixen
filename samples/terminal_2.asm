TTY_STDOUT = $04000200          ; Memory address of stdout
TTY_STDIN  = $04000204          ; Memory address of stdin
TTY_BUFFER = $04000208          ; Memory address of buffer

mov $04500aaa, #$000003b3

main:
        ; Write hello world to stack
        psh #'!'
        psh #'d'
        psh #'l'
        psh #'r'
        psh #'o'
        psh #'w'
        psh #' '
        psh #'o'
        psh #'l'
        psh #'l'
        psh #'e'
        psh #'H'

        ; Write prompt message
        mov {TTY_STDOUT}, #'P'  ; Display the message
        mov {TTY_STDOUT}, #'r'
        mov {TTY_STDOUT}, #'e'
        mov {TTY_STDOUT}, #'s'
        mov {TTY_STDOUT}, #'s'
        mov {TTY_STDOUT}, #' '
        mov {TTY_STDOUT}, #'a'
        mov {TTY_STDOUT}, #' '
        mov {TTY_STDOUT}, #'k'
        mov {TTY_STDOUT}, #'e'
        mov {TTY_STDOUT}, #'y'
        mov {TTY_STDOUT}, #'!'
        mov {TTY_STDOUT}, #$d   ; CR
        mov {TTY_STDOUT}, #$a   ; LF

        jmp loop

loop:
        jmp loop

interrupt:
        pll r0                  ; Get character from stack
        mov {TTY_STDOUT}, r0    ; Print it
        irt
