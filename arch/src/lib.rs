#![no_std]
extern crate alloc;

pub const CPU_SPECIFICATION: StaticSpecification = StaticSpecification {
    name: b"Floofi(TM) Vixen(TM) Kanto",
    id: 0x0003,
    microarchitecture: b"VXAv2.1",
    microarchitecture_name: b"Goupix",
    data_width: 32,
    address_width: 32,
    microcode: 0x0006
};

pub mod core;
pub mod cpu;
pub mod instructions;
pub mod devices;
pub mod memory_types;

pub use cpu::CPU;
pub use memory_types::*;
pub use devices::BusDevice;

use crate::core::Interrupt;
use crate::core::StaticSpecification;

pub type CPUResult<T> = Result<T, Interrupt>;
pub type InstructionResult = CPUResult<()>;
