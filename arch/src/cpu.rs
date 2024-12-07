pub mod system_stack;
pub mod user_stack;
pub mod decoder;

use alloc::boxed::Box;
pub use decoder::Decoder;
pub use system_stack::SystemStack;
pub use user_stack::UserStack;

use alloc::vec;
use alloc::vec::Vec;
use crate::core::Interrupt;
use crate::core::registers::RegisterId;
use crate::core::Registers;
use crate::core::registers::StatusRegister;
use crate::{CPUResult, InstructionResult, CPU_SPECIFICATION};

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
    pub stack_pointer: u32,
    pub program_counter: u32,
    pub status_register: StatusRegister,
    pub memory: Box<[u8]>,
    pub system_stack: Vec<u32>
}

impl CPU {
    pub fn load_rom(&mut self, rom: &[u8]) -> CPUResult<()> {
        let base_address = 0xe000_0000;
        let end_address = base_address + rom.len();
        let rom_region = base_address..end_address;
        self.memory[rom_region].copy_from_slice(rom);

        let specification: Vec<u8> = CPU_SPECIFICATION.into();
        let base_address = 0xffff_fe00;
        let end_address = base_address + specification.len();
        let specification_region = base_address..end_address;
        self.memory[specification_region].copy_from_slice(specification.as_slice());

        // Reset stack pointer to the start of the stack
        self.stack_pointer = 0x0000_0000;
        self.system_stack_save_state()?;

        Ok(())
    }

    #[must_use]
    pub fn get_register(&self, register_id: RegisterId) -> u32 {
        match register_id {
            RegisterId::R0 => self.registers.r0,
            RegisterId::R1 => self.registers.r1,
            RegisterId::R2 => self.registers.r2,
            RegisterId::R3 => self.registers.r3,
            RegisterId::R4 => self.registers.r4,
            RegisterId::R5 => self.registers.r5,
            RegisterId::R6 => self.registers.r6,
            RegisterId::R7 => self.registers.r7,
            RegisterId::R8 => self.registers.r8,
            RegisterId::R9 => self.registers.r9,
            RegisterId::R10 => self.registers.r10,
            RegisterId::R11 => self.registers.r11,
            RegisterId::R12 => self.registers.r12,
            RegisterId::R13 => self.registers.r13,
            RegisterId::R14 => self.registers.r14
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
            self.registers.r14 = interrupt.into();
            self.program_counter = 0xffff_0000;
        // Otherwise this is the first time we see an interrupt, so just use the configured handler
        } else {
            self.status_register.interrupt = true;
            let address = u32::from_le_bytes([
                self.memory[0x2000_0000], self.memory[0x2000_0001],
                self.memory[0x2000_0002], self.memory[0x2000_0003]
            ]);
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
            stack_pointer: 0x0000_0000,
            program_counter: 0xe000_0000,
            status_register: StatusRegister::default(),
            memory: vec![0u8; 0xffff_ffff].into_boxed_slice(),
            system_stack: vec![],
        }
    }
}
