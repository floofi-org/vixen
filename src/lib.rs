pub mod core;
pub mod cpu;

use crate::core::interrupt::Interrupt;
pub type CPUResult<T> = Result<T, Interrupt>;