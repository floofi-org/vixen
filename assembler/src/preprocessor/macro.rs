use std::collections::VecDeque;

use vixen::core::instruction::Operation;

use crate::{models::{Address, Instruction, Operand}, parser::Program};

pub enum Macro {
    Interrupt,
    DoubleFault,
}

impl Macro {
    // See: https://github.com/floofi-org/vixen/wiki/Memory-management-and-registers#memory-map
    const START_OF_BOOT_ROM: u32 = 0x0000_0200;

    // See: https://github.com/floofi-org/vixen/wiki/Interrupts-and-faults
    const INTERRUPT_HANDLER_ADDRESS: u32 = 0x0450_0aaa;
    const DOUBLE_FAULT_HANDLER_ADDRESS: u32 = 0x0400_dead;

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "interrupt" => Some(Self::Interrupt),
            "double_fault" => Some(Self::DoubleFault),
            _ => None,
        }
    }

    pub fn apply(&self, instructions: &mut VecDeque<Instruction>, instruction_offset: usize) {
        match self {
            Self::Interrupt => Self::interrupt(instructions, instruction_offset),
            Self::DoubleFault => Self::double_fault(instructions, instruction_offset),
        }
    }

    fn interrupt(instructions: &mut  VecDeque<Instruction>, instruction_offset: usize) {
        Self::define_handler(instructions, Self::INTERRUPT_HANDLER_ADDRESS, instruction_offset);
    }

    fn double_fault(instructions: &mut  VecDeque<Instruction>, instruction_offset: usize) {
        Self::define_handler(instructions, Self::DOUBLE_FAULT_HANDLER_ADDRESS, instruction_offset);
    }

    fn define_handler(instructions: &mut  VecDeque<Instruction>, setup_address: u32, handler_offset: usize) {
        let mov = Instruction {
            operation: Operation::Mov,
            operands: vec![
                Operand::Address(Address::Absolute(setup_address)),
                Operand::Literal(Self::START_OF_BOOT_ROM + handler_offset as u32),
            ],
        };

        instructions.push_front(mov);
    }

}
