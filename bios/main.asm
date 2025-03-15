CONFIG_TTY_STDOUT = $04000200          ; Memory address of terminal stdout
CONFIG_TTY_STDIN  = $04000204          ; Memory address of terminal stdin
CONFIG_TTY_BUFFER = $04000208          ; Memory address of terminal buffer

CONFIG_CPUNAME_LENGTH = $00000000      ; Memory address of CPU name length
CONFIG_CPUNAME_START = $00000001       ; Memory address of CPU name block

main:
    jmp bios_header
    jmp bios_cpuname
    jmp bios_cpuversion
    mov {CONFIG_TTY_STDOUT}, #$d
    mov {CONFIG_TTY_STDOUT}, #$a
    jmp bios_memory
    jmp bios_memorycheck
    mov {CONFIG_TTY_STDOUT}, #$d
    mov {CONFIG_TTY_STDOUT}, #$a
    jmp bios_enter
    int

bios_header:
    mov {CONFIG_TTY_STDOUT}, #'V'
    mov {CONFIG_TTY_STDOUT}, #'i'
    mov {CONFIG_TTY_STDOUT}, #'x'
    mov {CONFIG_TTY_STDOUT}, #'e'
    mov {CONFIG_TTY_STDOUT}, #'n'
    mov {CONFIG_TTY_STDOUT}, #' '
    mov {CONFIG_TTY_STDOUT}, #'K'
    mov {CONFIG_TTY_STDOUT}, #'i'
    mov {CONFIG_TTY_STDOUT}, #'c'
    mov {CONFIG_TTY_STDOUT}, #'k'
    mov {CONFIG_TTY_STDOUT}, #'s'
    mov {CONFIG_TTY_STDOUT}, #'t'
    mov {CONFIG_TTY_STDOUT}, #'a'
    mov {CONFIG_TTY_STDOUT}, #'r'
    mov {CONFIG_TTY_STDOUT}, #'t'
    mov {CONFIG_TTY_STDOUT}, #' '
    mov {CONFIG_TTY_STDOUT}, #'['
    mov {CONFIG_TTY_STDOUT}, #'R'
    mov {CONFIG_TTY_STDOUT}, #'e'
    mov {CONFIG_TTY_STDOUT}, #'v'
    mov {CONFIG_TTY_STDOUT}, #'i'
    mov {CONFIG_TTY_STDOUT}, #'s'
    mov {CONFIG_TTY_STDOUT}, #'i'
    mov {CONFIG_TTY_STDOUT}, #'o'
    mov {CONFIG_TTY_STDOUT}, #'n'
    mov {CONFIG_TTY_STDOUT}, #' '
    mov {CONFIG_TTY_STDOUT}, #'1'
    mov {CONFIG_TTY_STDOUT}, #']'
    mov {CONFIG_TTY_STDOUT}, #$d
    mov {CONFIG_TTY_STDOUT}, #$a
    ret

bios_cpuname:
    and r1, {CONFIG_CPUNAME_LENGTH}, #$FF ; CPU name length, AND to take only one byte
    mov r2, #{CONFIG_CPUNAME_START} ; Start of the CPU name
    mov r3, #{CONFIG_TTY_STDOUT} ; Copy to the start of the RAM
    jmp strcopy
    ret

bios_cpuversion:
    mov {CONFIG_TTY_STDOUT}, #' '
    mov {CONFIG_TTY_STDOUT}, #'['
    mov {CONFIG_TTY_STDOUT}, #'V'
    mov {CONFIG_TTY_STDOUT}, #'e'
    mov {CONFIG_TTY_STDOUT}, #'r'
    mov {CONFIG_TTY_STDOUT}, #'s'
    mov {CONFIG_TTY_STDOUT}, #'i'
    mov {CONFIG_TTY_STDOUT}, #'o'
    mov {CONFIG_TTY_STDOUT}, #'n'
    mov {CONFIG_TTY_STDOUT}, #' '
    mov {CONFIG_TTY_STDOUT}, #'X'
    mov {CONFIG_TTY_STDOUT}, #']'
    mov {CONFIG_TTY_STDOUT}, #$d
    mov {CONFIG_TTY_STDOUT}, #$a
    ret

bios_memory:
    mov {CONFIG_TTY_STDOUT}, #'X'
    mov {CONFIG_TTY_STDOUT}, #'K'
    mov {CONFIG_TTY_STDOUT}, #' '
    mov {CONFIG_TTY_STDOUT}, #'s'
    mov {CONFIG_TTY_STDOUT}, #'y'
    mov {CONFIG_TTY_STDOUT}, #'s'
    mov {CONFIG_TTY_STDOUT}, #'t'
    mov {CONFIG_TTY_STDOUT}, #'e'
    mov {CONFIG_TTY_STDOUT}, #'m'
    mov {CONFIG_TTY_STDOUT}, #' '
    mov {CONFIG_TTY_STDOUT}, #'m'
    mov {CONFIG_TTY_STDOUT}, #'e'
    mov {CONFIG_TTY_STDOUT}, #'m'
    mov {CONFIG_TTY_STDOUT}, #'o'
    mov {CONFIG_TTY_STDOUT}, #'r'
    mov {CONFIG_TTY_STDOUT}, #'y'
    mov {CONFIG_TTY_STDOUT}, #$d
    mov {CONFIG_TTY_STDOUT}, #$a
    ret

bios_memorycheck:
    mov {CONFIG_TTY_STDOUT}, #'O'
    mov {CONFIG_TTY_STDOUT}, #'K'
    mov {CONFIG_TTY_STDOUT}, #$d
    mov {CONFIG_TTY_STDOUT}, #$a
    ret

bios_enter:
    mov {CONFIG_TTY_STDOUT}, #'P'
    mov {CONFIG_TTY_STDOUT}, #'r'
    mov {CONFIG_TTY_STDOUT}, #'e'
    mov {CONFIG_TTY_STDOUT}, #'s'
    mov {CONFIG_TTY_STDOUT}, #'s'
    mov {CONFIG_TTY_STDOUT}, #' '
    mov {CONFIG_TTY_STDOUT}, #'E'
    mov {CONFIG_TTY_STDOUT}, #'n'
    mov {CONFIG_TTY_STDOUT}, #'t'
    mov {CONFIG_TTY_STDOUT}, #'e'
    mov {CONFIG_TTY_STDOUT}, #'r'
    mov {CONFIG_TTY_STDOUT}, #' '
    mov {CONFIG_TTY_STDOUT}, #'t'
    mov {CONFIG_TTY_STDOUT}, #'o'
    mov {CONFIG_TTY_STDOUT}, #' '
    mov {CONFIG_TTY_STDOUT}, #'c'
    mov {CONFIG_TTY_STDOUT}, #'o'
    mov {CONFIG_TTY_STDOUT}, #'n'
    mov {CONFIG_TTY_STDOUT}, #'t'
    mov {CONFIG_TTY_STDOUT}, #'i'
    mov {CONFIG_TTY_STDOUT}, #'n'
    mov {CONFIG_TTY_STDOUT}, #'u'
    mov {CONFIG_TTY_STDOUT}, #'e'
    mov {CONFIG_TTY_STDOUT}, #'.'
    mov {CONFIG_TTY_STDOUT}, #$d
    mov {CONFIG_TTY_STDOUT}, #$a
    jmp bios_enter_loop

bios_enter_loop:
    jmpl bios_enter_loop

.interrupt
bios_enter_interrupt:
    mov {CONFIG_TTY_STDOUT}, #'.'
    iret

; Arguments
; String length ->          R1
; Source address ->         R2
; Destination address ->    R3
strcopy:
    ; Check whether the string length is zero
    ; If so return
    cmp r1, #0
    jnz copy
    ret

    copy:
        ; Copy character to destination
        and [r3], [r2], #$FF

        ; Increment pointers and decrement remaining string length
        inc r2
        dec r1

        ; Move next
        jmpl strcopy