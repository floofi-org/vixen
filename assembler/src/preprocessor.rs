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
    NoSuchLabel(String),
    NoSuchMacro(String),
    UnexpectedMacroArguments(String, usize, usize),
    IncludeCompileError(PathBuf, Box<Error>)
}

pub struct ProcessedProgram {
    pub labels: HashMap<String, usize>,
    pub instructions: VecDeque<Instruction>,
}

impl Preprocessor {
    // See: https://github.com/floofi-org/vixen/wiki/Memory-management-and-registers#memory-map
    const START_OF_BOOT_ROM: u32 = 0x0000_0200;
    const INSTRUCTION_SIZE: u32 = 15;

    pub fn process(source_path: &Path, program: Program) -> Result<ProcessedProgram, PreprocessorError> {
        let Program {labels, macros, instructions } = program;
        let mut processed = ProcessedProgram {
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

        for operand in operands {
            if let Operand::Label(label) = operand {
                let address = Self::get_label_address(&processed.labels, label)?;
                *operand = Operand::Address(address);
            }
        }

        Ok(processed)
    }

    fn get_label_address(labels: &HashMap<String, usize>, label: &str) -> Result<Address, PreprocessorError> {
        let address = labels
            .get(label)
            .ok_or_else(|| PreprocessorError::NoSuchLabel(label.to_owned()))?;

        #[allow(clippy::cast_possible_truncation)]
        let address = Self::START_OF_BOOT_ROM + *address as u32 * Self::INSTRUCTION_SIZE;

        Ok(Address::Absolute(address))
    }
}
