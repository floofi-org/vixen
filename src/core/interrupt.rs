use std::fmt::{Display, Formatter};
use crate::cpu::CPU;
use crate::cpu::decoder::CPUDecoder;

#[derive(Debug, Clone, Copy)]
pub enum Interrupt {
    Rtc, AsyncIO, Hardware, External,
    Breakpoint, IllegalInstruction, IllegalMemory, DivideByZero,
    StackOverflow, StackUnderflow,
    User1, User2, User3, User4, User5, User6, User7, User8,
    User9, User10, User11, User12, User13, User14, User15, User16,
    Failure, Reset
}

impl Interrupt {
    pub fn stack_trace(self, mut cpu: CPU) {
        println!();
        println!("Unhandled interrupt: {self}");

        println!();
        println!("a = {a:0>2X}; x = {x:0>2X}; y = {y:0>2X}",
                 a = cpu.registers.a, x = cpu.registers.x, y = cpu.registers.y);
        println!("r0 = {r0:0>2X}; r1 = {r1:0>2X}; r2 = {r2:0>2X}; r3 = {r3:0>2X}; r4 = {r4:0>2X}; \
        r5 = {r5:0>2X}; r6 = {r6:0>2X}; r7 = {r7:0>2X}",
                 r0 = cpu.registers.r0, r1 = cpu.registers.r1, r2 = cpu.registers.r2,
                 r3 = cpu.registers.r3, r4 = cpu.registers.r4, r5 = cpu.registers.r5,
                 r6 = cpu.registers.r6, r7 = cpu.registers.r7);
        println!("sp = {sp:0>4X}; sr = {sr}",
                 sp = cpu.sp, sr = cpu.sr);

        println!();
        println!("pc = {pc:0>4X}: {state}: {disassembler}",
                 pc = cpu.pc, state = cpu.extract_instruction(cpu.pc),
                 disassembler = cpu.read_instruction_string(cpu.pc));

        println!();
        print!("zerop = ");

        for (index, byte) in cpu.memory[0x0000..0x00FF].iter().enumerate() {
            print!("{:0>2X}", byte);
            if index % 1 == 0 {
                print!(" ");
            }
            if (index + 1) % 32 == 0 {
                print!("\n        ");
            }
        }

        println!();
        print!("\nstack = ");

        for (index, byte) in cpu.memory[0x0100..0x01FF].iter().enumerate() {
            print!("{:0>2X}", byte);
            if index % 1 == 0 {
                print!(" ");
            }
            if (index + 1) % 32 == 0 {
                print!("\n        ");
            }
        }

        println!("\n            - _interrupt (????): <not handled>");

        let stack = &mut cpu.memory[0x0100..0x01FF];
        stack.reverse();
        let chunks = stack.chunks(2);
        for chunk in chunks {
            if chunk != [0, 0] && chunk != [0] {
                println!("            - ??: 0x{:0>2X}{:0>2X}", chunk[1], chunk[0]);
            }
        }
    }
}

impl Display for Interrupt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Interrupt::Rtc => write!(f, "0x00 (Real-time clock tick)"),
            Interrupt::AsyncIO => write!(f, "0x01 (Asynchronous I/O event)"),
            Interrupt::Hardware => write!(f, "0x02 (General hardware fault)"),
            Interrupt::External => write!(f, "0x03 (External hardware interrupt)"),
            Interrupt::Breakpoint => write!(f, "0x10 (Breakpoint hit)"),
            Interrupt::IllegalInstruction => write!(f, "0x11 (Illegal instruction)"),
            Interrupt::IllegalMemory => write!(f, "0x12 (Illegal memory access)"),
            Interrupt::DivideByZero => write!(f, "0x13 (Divide by zero)"),
            Interrupt::StackOverflow => write!(f, "0x20 (Stack overflow)"),
            Interrupt::StackUnderflow => write!(f, "0x21 (Stack underflow)"),
            Interrupt::User1 => write!(f, "0xE0 (User-defined interrupt 1)"),
            Interrupt::User2 => write!(f, "0xE1 (User-defined interrupt 2)"),
            Interrupt::User3 => write!(f, "0xE2 (User-defined interrupt 3)"),
            Interrupt::User4 => write!(f, "0xE3 (User-defined interrupt 4)"),
            Interrupt::User5 => write!(f, "0xE4 (User-defined interrupt 5)"),
            Interrupt::User6 => write!(f, "0xE5 (User-defined interrupt 6)"),
            Interrupt::User7 => write!(f, "0xE6 (User-defined interrupt 7)"),
            Interrupt::User8 => write!(f, "0xE7 (User-defined interrupt 8)"),
            Interrupt::User9 => write!(f, "0xE8 (User-defined interrupt 9)"),
            Interrupt::User10 => write!(f, "0xE9 (User-defined interrupt 10)"),
            Interrupt::User11 => write!(f, "0xEA (User-defined interrupt 11)"),
            Interrupt::User12 => write!(f, "0xEB (User-defined interrupt 12)"),
            Interrupt::User13 => write!(f, "0xEC (User-defined interrupt 13)"),
            Interrupt::User14 => write!(f, "0xED (User-defined interrupt 14)"),
            Interrupt::User15 => write!(f, "0xEE (User-defined interrupt 15)"),
            Interrupt::User16 => write!(f, "0xEF (User-defined interrupt 16)"),
            Interrupt::Failure => write!(f, "0xFE (Internal system failure)"),
            Interrupt::Reset => write!(f, "0xFF (System reset)")
        }
    }
}