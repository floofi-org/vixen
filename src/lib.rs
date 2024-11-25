#![no_std]

pub mod core;
pub mod cpu;
mod instructions;

use crate::core::interrupt::Interrupt;
pub type CPUResult<T> = Result<T, Interrupt>;
pub type CPUInstructionResult = CPUResult<()>;