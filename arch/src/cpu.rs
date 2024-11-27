pub mod stack;
pub mod decoder;

use crate::core::register_id::RegisterId;
use crate::core::registers::Registers;
use crate::core::status_register::StatusRegister;
use crate::cpu::stack::Stack;

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
    pub stack_pointer: u16,
    pub program_counter: u16,
    pub status_register: StatusRegister,
    pub memory: [u8; 0xFFFF]
}

impl CPU {
    pub fn load_rom(&mut self, rom: &[u8]) {
        let base_address = 0xE000;
        let end_address = 0xE000 + rom.len();
        let rom_region = base_address..end_address;

        self.memory[rom_region].copy_from_slice(rom);

        // Reset stack pointer to the start of the stack
        self.stack_pointer = 0x0100;
        self.stack_push_dword(0xE000).unwrap();
    }

    pub fn get_register(&self, register_id: RegisterId) -> u8 {
        match register_id {
            RegisterId::A => self.registers.a,
            RegisterId::X => self.registers.x,
            RegisterId::Y => self.registers.y,
            RegisterId::R0 => self.registers.r0,
            RegisterId::R1 => self.registers.r1,
            RegisterId::R2 => self.registers.r2,
            RegisterId::R3 => self.registers.r3,
            RegisterId::R4 => self.registers.r4,
            RegisterId::R5 => self.registers.r5,
            RegisterId::R6 => self.registers.r6,
            RegisterId::R7 => self.registers.r7
        }
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            registers: Registers::default(),
            stack_pointer: 0x0100,
            program_counter: 0xE000,
            status_register: StatusRegister::default(),
            memory: [0; 0xFFFF]
        }
    }
}
