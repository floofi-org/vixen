TTY_STDOUT = $04000200          ; Memory address of stdout
TTY_STDIN  = $04000204          ; Memory address of stdin
TTY_BUFFER = $04000208          ; Memory address of buffer

mov $04500200, #$000002ff

main:
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
        jmpl loop

interrupt:
        jmp interrupt_print
        iret

interrupt_print:
        mov {TTY_STDOUT}, {TTY_STDIN}   ; Print character from buffer
        cmp {TTY_BUFFER}, #$1           ; Check if buffer is empty
        jnz +30                         ; Return if empty
        jmpl interrupt_print             ; Print again if not empty
        ret
