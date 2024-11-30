#![no_std]
extern crate alloc;

pub const CPU_SPECIFICATION: Specification = Specification {
    name: "Floofi(TM) Vixen(TM) Arctic",
    id: 0x0001,
    microarchitecture: "vx1",
    microarchitecture_name: "Goupil",
    data_width: 8,
    address_width: 16,
    max_ram: 52735,
    microcode: 0x0001
};

pub mod core;
pub mod cpu;
pub mod instructions;

use crate::core::interrupt::Interrupt;
use crate::core::specification::Specification;

pub type CPUResult<T> = Result<T, Interrupt>;
pub type InstructionResult = CPUResult<()>;
