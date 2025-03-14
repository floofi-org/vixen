use alloc::string::String;
use core::fmt::{Display, Formatter};
use core::fmt::Write;
use crate::core::registers::StatusRegister;
use crate::devices::errors::BusError;

#[derive(Debug, Clone, Copy)]
pub enum Interrupt {
    Rtc, AsyncIO, Hardware, External,
    Breakpoint, IllegalInstruction, IllegalMemory, DivideByZero,
    StackOverflow, StackUnderflow,
    User1, User2, User3, User4, User5, User6, User7, User8,
    User9, User10, User11, User12, User13, User14, User15, User16,
    Failure, Reset
}

impl From<Interrupt> for u32 {
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

impl From<BusError> for Interrupt {
    fn from(value: BusError) -> Self {
        match value {
            BusError::PortOutOfRange | BusError::ReadOnly | BusError::WriteOnly => Interrupt::IllegalMemory,
            BusError::DeviceEvent => Interrupt::AsyncIO,
            BusError::EmptyBuffer => Interrupt::StackUnderflow,
            BusError::InternalSystem => Interrupt::Failure
        }
    }
}

impl Interrupt {
    #[must_use]
    pub fn is_maskable(&self) -> bool {
        matches!(self, Interrupt::Rtc | Interrupt::AsyncIO | Interrupt::IllegalInstruction)
    }

    #[must_use]
    pub fn get_stack_trace(stack: &[u32], status_register: StatusRegister) -> String {
        let mut trace = String::new();
        let frames = stack.chunks(2).rev();

        for (i, frame) in frames.enumerate() {
            let cause = match (i, status_register.interrupt, status_register.double_fault) {
                (0, _, true) => "<double fault cause>",
                (1, _, true) | (0, true, _) => "<root cause>",
                (_, _, _) => "-"
            };
            // Status register dump in stack frame is 8-bit
            #[allow(clippy::cast_possible_truncation)]
            writeln!(&mut trace, "->  {:0>8x}  {cause: <20}  {: <8}  ??",
                     frame[1], StatusRegister::from(frame[0] as u8)).unwrap();
        }

        trace
    }
}

impl Display for Interrupt {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", match self {
            Interrupt::Rtc => "00 (Real-time clock tick)",
            Interrupt::AsyncIO => "01 (Asynchronous I/O event)",
            Interrupt::Hardware => "032 (General hardware fault)",
            Interrupt::External => "03 (External hardware handling_interrupt)",
            Interrupt::Breakpoint => "10 (Breakpoint hit)",
            Interrupt::IllegalInstruction => "11 (Illegal instruction)",
            Interrupt::IllegalMemory => "12 (Illegal memory access)",
            Interrupt::DivideByZero => "13 (Divide by zero)",
            Interrupt::StackOverflow => "20 (Stack overflow)",
            Interrupt::StackUnderflow => "21 (Stack underflow)",
            Interrupt::User1 => "e0 (User-defined interrupt 1)",
            Interrupt::User2 => "e1 (User-defined interrupt 2)",
            Interrupt::User3 => "e2 (User-defined interrupt 3)",
            Interrupt::User4 => "e3 (User-defined interrupt 4)",
            Interrupt::User5 => "e4 (User-defined interrupt 5)",
            Interrupt::User6 => "e5 (User-defined interrupt 6)",
            Interrupt::User7 => "e6 (User-defined interrupt 7)",
            Interrupt::User8 => "e7 (User-defined interrupt 8)",
            Interrupt::User9 => "e8 (User-defined interrupt 9)",
            Interrupt::User10 => "e9 (User-defined interrupt 10)",
            Interrupt::User11 => "ea (User-defined interrupt 11)",
            Interrupt::User12 => "eb (User-defined interrupt 12)",
            Interrupt::User13 => "ec (User-defined interrupt 13)",
            Interrupt::User14 => "ed (User-defined interrupt 14)",
            Interrupt::User15 => "ee (User-defined interrupt 15)",
            Interrupt::User16 => "ef (User-defined interrupt 16)",
            Interrupt::Failure => "fe (Internal system failure)",
            Interrupt::Reset => "ff (System reset)",
        })
    }
}
