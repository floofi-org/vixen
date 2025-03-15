bios_cpuname:
    and r1, {CONFIG_CPUNAME_LENGTH}, #$FF ; CPU name length, AND to take only one byte
    mov r2, #{CONFIG_CPUNAME_START} ; Start of the CPU name
    mov r3, #{CONFIG_TTY_STDOUT} ; Copy to the start of the RAM
    jmp strcopy
    ret

.include "strcopy.asm"
