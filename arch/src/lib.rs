//#![no_std]
extern crate alloc;

pub const CPU_SPECIFICATION: StaticSpecification = StaticSpecification {
    name: b"Floofi(TM) Vixen(TM) Coyote",
    id: 0x0002,
    microarchitecture: b"vx2",
    microarchitecture_name: b"Kitsune",
    data_width: 32,
    address_width: 32,
    microcode: 0x0005
};

pub mod core;
pub mod cpu;
pub mod instructions;
pub mod memory_types;

pub use cpu::CPU;
pub use memory_types::*;

use crate::core::Interrupt;
use crate::core::StaticSpecification;

pub type CPUResult<T> = Result<T, Interrupt>;
pub type InstructionResult = CPUResult<()>;
