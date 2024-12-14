use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};

use r#macro::Macro;

use crate::error::Error;
use crate::parser::Program;
use crate::models::{Address, Instruction, Operand};

mod r#macro;

pub struct Preprocessor;

#[derive(Debug)]
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

        Ok(Address::Absolute(*address as u32))
    }
}
