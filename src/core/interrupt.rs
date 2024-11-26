use std::fmt::{Display, Formatter};
use std::fmt::Write;
use crate::cpu::CPU;
use crate::cpu::decoder::Decoder;

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
    pub fn stack_trace(self, cpu: CPU) {
        let zerop = Self::get_byte_dump(&cpu.memory[0x0000..0x0100]);
        let stack = Self::get_byte_dump(&cpu.memory[0x0100..0x0200]);
        let stack_trace = Self::get_stack_trace(&cpu.memory[0x0100..0x01FF]);

        println!(
            include!("interrupt/stack_trace.rs"),
            interrupt = self,
            a = cpu.registers.a,
            x = cpu.registers.x,
            y = cpu.registers.y,
            r0 = cpu.registers.r0,
            r1 = cpu.registers.r1,
            r2 = cpu.registers.r2,
            r3 = cpu.registers.r3,
            r4 = cpu.registers.r4,
            r5 = cpu.registers.r5,
            r6 = cpu.registers.r6,
            r7 = cpu.registers.r7,
            sp = cpu.sp,
            sr = cpu.sr,
            pc = cpu.pc,
            state = cpu.extract_instruction(cpu.pc),
            disassembler = cpu.read_instruction_string(cpu.pc),
            zerop = zerop,
            stack = stack,
            stack_trace = stack_trace,
        )
    }

    // I have no idea what goes on with these two
    fn get_byte_dump(bytes: &[u8]) -> String {
        let mut dump = String::new();

        for (index, byte) in bytes.iter().enumerate() {
            write!(&mut dump, "{:0>2X}", byte).unwrap();
            write!(&mut dump, " ").unwrap();

            if (index + 1) % 32 == 0 && index != 255 {
                write!(&mut dump, "\n        ").unwrap();
            }
        }

        dump
    }

    fn get_stack_trace(stack: &[u8]) -> String {
        let mut trace = String::new();
        let frames = stack.chunks(2).rev();

        for frame in frames {
            if frame != [0, 0] && frame != [0] {
                writeln!(&mut trace, "            - ??: 0x{:0>2X}{:0>2X}", frame[1], frame[0]).unwrap();
            }
        }

        trace
    }
}

impl Display for Interrupt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Interrupt::Rtc => "0x00 (Real-time clock tick)",
            Interrupt::AsyncIO => "0x01 (Asynchronous I/O event)",
            Interrupt::Hardware => "0x02 (General hardware fault)",
            Interrupt::External => "0x03 (External hardware interrupt)",
            Interrupt::Breakpoint => "0x10 (Breakpoint hit)",
            Interrupt::IllegalInstruction => "0x11 (Illegal instruction)",
            Interrupt::IllegalMemory => "0x12 (Illegal memory access)",
            Interrupt::DivideByZero => "0x13 (Divide by zero)",
            Interrupt::StackOverflow => "0x20 (Stack overflow)",
            Interrupt::StackUnderflow => "0x21 (Stack underflow)",
            Interrupt::User1 => "0xE0 (User-defined interrupt 1)",
            Interrupt::User2 => "0xE1 (User-defined interrupt 2)",
            Interrupt::User3 => "0xE2 (User-defined interrupt 3)",
            Interrupt::User4 => "0xE3 (User-defined interrupt 4)",
            Interrupt::User5 => "0xE4 (User-defined interrupt 5)",
            Interrupt::User6 => "0xE5 (User-defined interrupt 6)",
            Interrupt::User7 => "0xE6 (User-defined interrupt 7)",
            Interrupt::User8 => "0xE7 (User-defined interrupt 8)",
            Interrupt::User9 => "0xE8 (User-defined interrupt 9)",
            Interrupt::User10 => "0xE9 (User-defined interrupt 10)",
            Interrupt::User11 => "0xEA (User-defined interrupt 11)",
            Interrupt::User12 => "0xEB (User-defined interrupt 12)",
            Interrupt::User13 => "0xEC (User-defined interrupt 13)",
            Interrupt::User14 => "0xED (User-defined interrupt 14)",
            Interrupt::User15 => "0xEE (User-defined interrupt 15)",
            Interrupt::User16 => "0xEF (User-defined interrupt 16)",
            Interrupt::Failure => "0xFE (Internal system failure)",
            Interrupt::Reset => "0xFF (System reset)",
        };

        write!(f, "{}", str)
    }
}
