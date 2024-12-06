pub mod system_stack;
pub mod user_stack;
pub mod decoder;

use alloc::vec;
use alloc::vec::Vec;
use crate::core::interrupt::Interrupt;
use crate::core::registers::register_id::RegisterId;
use crate::core::registers::Registers;
use crate::core::registers::status_register::StatusRegister;
use crate::cpu::decoder::Decoder;
use crate::cpu::system_stack::SystemStack;
use crate::{CPUResult, InstructionResult, CPU_SPECIFICATION};

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
    pub stack_pointer: u16,
    pub program_counter: u16,
    pub status_register: StatusRegister,
    pub memory: [u8; 0xFFFF],
    pub system_stack: Vec<u16>
}

impl CPU {
    pub fn load_rom(&mut self, rom: &[u8]) -> CPUResult<()> {
        let base_address = 0xE000;
        let end_address = base_address + rom.len();
        let rom_region = base_address..end_address;
        self.memory[rom_region].copy_from_slice(rom);

        let specification: Vec<u8> = CPU_SPECIFICATION.into();
        let base_address = 0xFE00;
        let end_address = base_address + specification.len();
        let specification_region = base_address..end_address;
        self.memory[specification_region].copy_from_slice(specification.as_slice());

        // Reset stack pointer to the start of the stack
        self.stack_pointer = 0x0100;
        self.system_stack_save_state()?;
        
        Ok(())
    }

    #[must_use] pub fn get_register(&self, register_id: RegisterId) -> u8 {
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

    fn has_interrupt_handler(&self) -> bool {
        self.memory[0x00FE] != 0 || self.memory[0x00FF] != 0
    }

    pub fn tick_unhandled(&mut self) -> InstructionResult {
        let mut instruction = self.read_instruction(self.program_counter)?;
        if let Err(interrupt) = instruction.execute_unhandled(self) {
            if self.status_register.interrupt_disable && interrupt.is_maskable() {
                Ok(())
            } else {
                Err(interrupt)
            }
        } else {
            Ok(())
        }
    }

    fn handle_interrupt(&mut self, interrupt: Interrupt) -> InstructionResult {
        self.system_stack_save_state()?;

        // If we are already handling interrupt, use ROM-provided double fault handler
        if self.status_register.interrupt {
            self.status_register.double_fault = true;
            self.registers.r7 = interrupt.into();
            self.program_counter = 0xF0F0;
        // Otherwise this is the first time we see an interrupt, so just use the configured handler
        } else {
            self.status_register.interrupt = true;
            let address = u16::from_le_bytes([self.memory[0x00FE], self.memory[0x00FF]]);
            self.program_counter = address;
        }

        Ok(())
    }

    pub fn tick(&mut self) -> InstructionResult {
        // If we are in a triple fault or no interrupt handler is configured
        if self.status_register.double_fault || !self.has_interrupt_handler() {
            self.tick_unhandled()
        } else {
            match self.tick_unhandled() {
                Ok(()) => Ok(()),
                Err(interrupt) => {
                    self.handle_interrupt(interrupt)?;
                    self.tick()
                }
            }
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
            memory: [0; 0xFFFF],
            system_stack: vec![],
        }
    }
}
