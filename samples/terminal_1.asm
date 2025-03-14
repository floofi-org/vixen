TTY_STDOUT = $04000200          ; Memory address of stdout

main:
        mov {TTY_STDOUT}, #'H'  ; Display the message
        mov {TTY_STDOUT}, #'e'
        mov {TTY_STDOUT}, #'l'
        mov {TTY_STDOUT}, #'l'
        mov {TTY_STDOUT}, #'o'
        mov {TTY_STDOUT}, #' '
        mov {TTY_STDOUT}, #'w'
        mov {TTY_STDOUT}, #'o'
        mov {TTY_STDOUT}, #'r'
        mov {TTY_STDOUT}, #'l'
        mov {TTY_STDOUT}, #'d'
        mov {TTY_STDOUT}, #'!'
        mov {TTY_STDOUT}, #$d   ; CR
        mov {TTY_STDOUT}, #$a   ; LF
        int
