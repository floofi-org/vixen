#![no_std]
extern crate alloc;

pub const CPU_SPECIFICATION: Specification = Specification {
    name: b"Floofi(TM) Vixen(TM) Coyote",
    id: 0x0002,
    microarchitecture: b"vx2",
    microarchitecture_name: b"Kitsune",
    data_width: 32,
    address_width: 32,
    max_ram: 4_294_967_295,
    microcode: 0x0003
};

pub mod core;
pub mod cpu;
pub mod instructions;
pub mod devices;

pub use cpu::CPU;
pub use devices::BusDevice;

use crate::core::Interrupt;
use crate::core::Specification;

pub type CPUResult<T> = Result<T, Interrupt>;
pub type InstructionResult = CPUResult<()>;
