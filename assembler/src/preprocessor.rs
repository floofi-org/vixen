use std::collections::{HashMap, VecDeque};

use r#macro::Macro;

use crate::parser::Program;
use crate::models::{Address, Instruction, Operand};

mod r#macro;

pub struct Preprocessor;

#[derive(Debug)]
pub enum PreprocessorError {
    NoSuchLabel(String),
    NoSuchMacro(String),
    UnexpectedMacroArguments(String, usize, usize),
}

impl Preprocessor {
    pub fn process(mut program: Program) -> Result<VecDeque<Instruction>, PreprocessorError> {
        let operands = program.instructions
            .iter_mut()
            .flat_map(|i| i.operands.iter_mut());

        for operand in operands {
            if let Operand::Label(label) = operand {
                let address = Self::get_label_address(&program.labels, label)?;
                *operand = Operand::Address(address);
            }
        }

        for (definition, offset) in program.macros {
            let r#macro: Macro = definition.try_into()?;
            r#macro.apply(&mut program.instructions, offset);
        }

        Ok(program.instructions)
    }

    fn get_label_address(labels: &HashMap<String, usize>, label: &str) -> Result<Address, PreprocessorError> {
        let address = labels
            .get(label)
            .ok_or_else(|| PreprocessorError::NoSuchLabel(label.to_owned()))?;

        Ok(Address::Absolute(*address as u32))
    }
}
