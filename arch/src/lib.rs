#![no_std]
extern crate alloc;

pub mod core;
pub mod cpu;
pub mod instructions;

use crate::core::interrupt::Interrupt;
pub type CPUResult<T> = Result<T, Interrupt>;
pub type InstructionResult = CPUResult<()>;
