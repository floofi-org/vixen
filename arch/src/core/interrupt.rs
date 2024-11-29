use alloc::string::String;
use core::fmt::{Display, Formatter};
use core::fmt::Write;
use crate::core::registers::status_register::StatusRegister;

#[derive(Debug, Clone, Copy)]
pub enum Interrupt {
    Rtc, AsyncIO, Hardware, External,
    Breakpoint, IllegalInstruction, IllegalMemory, DivideByZero,
    StackOverflow, StackUnderflow,
    User1, User2, User3, User4, User5, User6, User7, User8,
    User9, User10, User11, User12, User13, User14, User15, User16,
    Failure, Reset
}

impl From<Interrupt> for u8 {
    fn from(value: Interrupt) -> Self {
        match value {
            Interrupt::Rtc => 0x00,
            Interrupt::AsyncIO => 0x01,
            Interrupt::Hardware => 0x02,
            Interrupt::External => 0x03,
            Interrupt::Breakpoint => 0x10,
            Interrupt::IllegalInstruction => 0x11,
            Interrupt::IllegalMemory => 0x12,
            Interrupt::DivideByZero => 0x13,
            Interrupt::StackOverflow => 0x20,
            Interrupt::StackUnderflow => 0x21,
            Interrupt::User1 => 0xE0,
            Interrupt::User2 => 0xE1,
            Interrupt::User3 => 0xE2,
            Interrupt::User4 => 0xE3,
            Interrupt::User5 => 0xE4,
            Interrupt::User6 => 0xE5,
            Interrupt::User7 => 0xE6,
            Interrupt::User8 => 0xE7,
            Interrupt::User9 => 0xE8,
            Interrupt::User10 => 0xE9,
            Interrupt::User11 => 0xEA,
            Interrupt::User12 => 0xEB,
            Interrupt::User13 => 0xEC,
            Interrupt::User14 => 0xED,
            Interrupt::User15 => 0xEE,
            Interrupt::User16 => 0xEF,
            Interrupt::Failure => 0xFE,
            Interrupt::Reset => 0xFF,
        }
    }
}

impl Interrupt {
    pub fn is_maskable(&self) -> bool {
        matches!(self, Interrupt::Rtc | Interrupt::AsyncIO | Interrupt::IllegalInstruction)
    }

    pub fn get_byte_dump(bytes: &[u8], line_size: usize, padding: usize) -> String {
        let mut dump = String::new();

        for (index, byte) in bytes.iter().enumerate() {
            write!(&mut dump, "{:0>2X}", byte).unwrap();
            write!(&mut dump, " ").unwrap();

            if (index + 1) % line_size == 0 && index != bytes.len() - 1 {
                write!(&mut dump, "\n{}", " ".repeat(padding)).unwrap();
            }
        }

        dump
    }

    pub fn get_stack_trace(stack: &[u16], status_register: StatusRegister) -> String {
        let mut trace = String::new();
        let frames = stack.chunks(2).rev();

        for (i, frame) in frames.enumerate() {
            let cause = match (i, status_register.interrupt, status_register.double_fault) {
                (0, _, true) => "<double fault cause>",
                (1, _, true) | (0, true, _) => "<root cause>",
                (_, _, _) => "-"
            };
            writeln!(&mut trace, "->  0x{:0>4X}  {cause: <20}  {: <8}  ??",
                     frame[1], StatusRegister::from(frame[0] as u8)).unwrap();
        }

        trace
    }
}

impl Display for Interrupt {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", match self {
            Interrupt::Rtc => "0x00 (Real-time clock tick)",
            Interrupt::AsyncIO => "0x01 (Asynchronous I/O event)",
            Interrupt::Hardware => "0x02 (General hardware fault)",
            Interrupt::External => "0x03 (External hardware handling_interrupt)",
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
        })
    }
}
