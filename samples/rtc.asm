RTC_SECS = $0400020C
RTC_NANOS = $04000210
RTC_TIMER_SECS = $04000214
RTC_TIMER_NANOS = $04000218
RTC_TIMER_CLEAR = $0400021C

mov r0, {RTC_SECS} ; Read seconds
mov r1, {RTC_NANOS} ; Read nanoseconds

mov {RTC_TIMER_SECS}, #10 ; Set up a timer for 10 seconds

; Spin lock while we're waiting
loop:
    jmp loop
