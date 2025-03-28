use std::path::{Path, PathBuf};

use vixen::core::instruction::Operation;

use crate::models::{Address, Instruction, Operand};
use crate::parser::{MacroArg, MacroDefinition};

use super::{PreprocessorError, ProcessedProgram};

pub enum Macro {
    Interrupt,
    DoubleFault,
    Include(PathBuf),
}

impl Macro {
    // See: https://github.com/floofi-org/vixen/wiki/Memory-management-and-registers#memory-map
    const START_OF_BOOT_ROM: u32 = 0x0000_0200;

    // See: https://github.com/floofi-org/vixen/wiki/Interrupts-and-faults
    const INTERRUPT_HANDLER_ADDRESS: u32 = 0x0450_0200;
    const DOUBLE_FAULT_HANDLER_ADDRESS: u32 = 0x0450_0204;

    #[allow(clippy::unit_arg)]
    pub fn apply(self, source_path: &Path, program: &mut ProcessedProgram, instruction_offset: usize) -> Result<(), PreprocessorError> {
        match self {
            Self::Interrupt => Ok(Self::interrupt(program, instruction_offset)),
            Self::DoubleFault => Ok(Self::double_fault(program, instruction_offset)),
            Self::Include(path) => Self::include(program, instruction_offset, source_path, path),
        }
    }

    fn interrupt(program: &mut ProcessedProgram, instruction_offset: usize) {
        Self::define_handler(program, Self::INTERRUPT_HANDLER_ADDRESS, instruction_offset);
    }

    fn double_fault(program: &mut ProcessedProgram, instruction_offset: usize) {
        Self::define_handler(program, Self::DOUBLE_FAULT_HANDLER_ADDRESS, instruction_offset);
    }

    fn include(program: &mut ProcessedProgram, instruction_offset: usize, source_path: &Path, path: PathBuf) -> Result<(), PreprocessorError> {
        let source_path = source_path.parent().unwrap().join(path);
        let source = match std::fs::read_to_string(&source_path) {
            Ok(s) => s,
            Err(e) => {
                return Err(PreprocessorError::IncludeCompileError(source_path, Box::new(e.into())));
            }
        };

        let mut included = match crate::compile_for_include(&source_path, &source) {
            Ok(s) => s,
            Err(e) => {
                return Err(PreprocessorError::IncludeCompileError(source_path, Box::new(e)));
            }
        };

        // Reserve capacity upfront
        program.instructions.reserve(included.instructions.len());

        // Going in reverse is important here
        // Since we push instructions downwards, so we need to start from the bottom
        for instruction in included.instructions.into_iter().rev() {
            program.instructions.insert(instruction_offset, instruction);
        }

        for label in &mut included.labels {
            *label.1 += instruction_offset;
        }

        program.labels.extend(included.labels);
        program.constants.extend(included.constants);

        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    fn define_handler(program: &mut ProcessedProgram, setup_address: u32, handler_offset: usize) {
        let mov = Instruction {
            operation: Operation::Mov,
            operands: vec![
                Operand::Address(Address::Absolute(setup_address)),
                Operand::Literal(Self::START_OF_BOOT_ROM + handler_offset as u32),
            ],
        };

        // Every label is gonna have to move by 1
        for label in &mut program.labels {
            *label.1 += 1;
        }

        program.instructions.push_front(mov);
    }

}

impl TryFrom<MacroDefinition> for Macro {
    type Error = PreprocessorError;

    fn try_from(value: MacroDefinition) -> Result<Self, Self::Error> {
        let MacroDefinition { name, args } = value;

        match name.as_str() {
            "interrupt" => Ok(Self::Interrupt),
            "double_fault" => Ok(Self::DoubleFault),
            "include" => macro_with_args::<1, _>("include", args, |args| Self::Include(PathBuf::from(args[0].0.clone()))),
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
