use std::collections::VecDeque;

use vixen::core::instruction::Operation;

use crate::models::{Address, Instruction, Operand};
use crate::parser::{MacroArg, MacroDefinition};

use super::PreprocessorError;

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

impl TryFrom<MacroDefinition> for Macro {
    type Error = PreprocessorError;

    fn try_from(value: MacroDefinition) -> Result<Self, Self::Error> {
        let MacroDefinition { name, args } = value;

        match name.as_str() {
            "interrupt" => Ok(Self::Interrupt),
            "double_fault" => Ok(Self::DoubleFault),
            _ => Err(PreprocessorError::NoSuchMacro(name)),
        }
    }
}

fn macro_with_args<const N: usize, F>(name: &str, args: Vec<MacroArg>, f: F) -> Result<Macro, PreprocessorError>
where
    F: FnOnce([MacroArg; N]) -> Macro,
{
    let args: [MacroArg; N] = match args.try_into() {
        Ok(args) => args,
        Err(args) => {
            return Err(PreprocessorError::UnexpectedMacroArguments(name.to_string(), args.len(), N))
        },
    };

    Ok(f(args))
}
