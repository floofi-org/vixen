use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};

use r#macro::Macro;

use crate::error::Error;
use crate::parser::Program;
use crate::models::{Address, Instruction, Operand};

mod r#macro;

pub struct Preprocessor;

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum PreprocessorError {
    NoSuchConstant(String),
    NoSuchLabel(String),
    NoSuchMacro(String),
    UnexpectedMacroArguments(String, usize, usize),
    IncludeCompileError(PathBuf, Box<Error>)
}

pub struct ProcessedProgram {
    pub constants: HashMap<String, u32>,
    pub labels: HashMap<String, usize>,
    pub instructions: VecDeque<Instruction>,
}

impl Preprocessor {
    // See: https://github.com/floofi-org/vixen/wiki/Memory-management-and-registers#memory-map
    const START_OF_BOOT_ROM: u32 = 0x0000_0200;
    const INSTRUCTION_SIZE: u32 = 15;

    pub fn process(source_path: &Path, program: Program) -> Result<ProcessedProgram, PreprocessorError> {
        let Program { constants, labels, macros, instructions } = program;
        let mut processed = ProcessedProgram {
            constants,
            labels,
            instructions,
        };

        for (definition, offset) in macros {
            let r#macro: Macro = definition.try_into()?;
            r#macro.apply(source_path, &mut processed, offset)?;
        }

        let operands = processed.instructions
            .iter_mut()
            .flat_map(|i| i.operands.iter_mut());

        // Transform constants and labels
        for operand in operands {
            match operand {
                Operand::ConstantLiteral(c) => *operand = Self::transform_constant(&processed.constants, c, true)?,
                Operand::ConstantAddress(c) => *operand = Self::transform_constant(&processed.constants, c, false)?,
                Operand::Label(label) => *operand = Self::transform_label(&processed.labels, label)?,
                _ => {}
            }
        }


        Ok(processed)
    }

    fn transform_constant(constants: &HashMap<String, u32>, constant: &str, is_literal: bool) -> Result<Operand, PreprocessorError> {
        let constant = *constants.get(constant)
            .ok_or_else(|| PreprocessorError::NoSuchConstant(constant.to_owned()))?;

        let operand = if is_literal {
            Operand::Literal(constant)
        } else {
            Operand::Address(Address::Absolute(constant))
        };

        Ok(operand)
    }

    fn transform_label(labels: &HashMap<String, usize>, label: &str) -> Result<Operand, PreprocessorError> {
        let address = *labels
            .get(label)
            .ok_or_else(|| PreprocessorError::NoSuchLabel(label.to_owned()))?;

        let address: u32 = address.try_into()
            .expect("Label address is too high for ROM");

        Ok(Self::get_label_address(address))
    }

    fn get_label_address(offset: u32) -> Operand {
        let address = Self::START_OF_BOOT_ROM + offset * Self::INSTRUCTION_SIZE;
        Operand::Address(Address::Absolute(address))
    }
}
