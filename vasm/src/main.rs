use vasm::scanner::Scanner;

const PROGRAM: &str = "
main:                   ; Our main program
        add 1, 1        ; Run some arbitrary calculation
        pll X           ; This should cause a stack underflow
        jmp main        ; Otherwise repeat infinitely

.interrupt              ; Define a custom interrupt handler
handle_interrupt:
        jam             ; Force the CPU to halt
";

fn main() {
    let tokens = Scanner::new(PROGRAM).scan();

    println!("{PROGRAM}");
    println!();
    println!("{tokens:#?}");
}
