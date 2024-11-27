pub mod stack;
pub mod decoder;

use crate::core::registers::register_id::RegisterId;
use crate::core::registers::Registers;
use crate::core::registers::status_register::StatusRegister;
use crate::cpu::decoder::Decoder;
use crate::cpu::stack::Stack;
use crate::InstructionResult;

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

    pub fn tick_unhandled(&mut self) -> InstructionResult {
        let mut instruction = self.read_instruction(self.program_counter)?;
        instruction.execute_unhandled(self)
    }

    pub fn tick(&mut self) -> InstructionResult {
        if self.status_register.double_fault || (self.memory[0x00FE] == 0 && self.memory[0x00FF] == 0) {
            self.tick_unhandled()
        } else if self.tick_unhandled().is_err() {
            self.stack_push_dword(self.program_counter)?;
            if self.status_register.interrupt {
                self.status_register.double_fault = true;
                self.program_counter = 0xF0F0;
            } else {
                self.status_register.interrupt = true;
                let address = u16::from_le_bytes([self.memory[0x00FE], self.memory[0x00FF]]);
                self.program_counter = address;
            }
            self.tick()
        } else {
            Ok(())
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
