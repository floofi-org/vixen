use crate::models::Instruction;

mod instruction;
mod operand;
mod operation;

pub trait Compile {
    fn compile(self, compiler: &mut Compiler) -> Result<(), CompilerError>;
}

#[derive(Debug)]
pub enum CompilerError {
    TooManyOperands,
}

#[derive(Default)]
pub struct Compiler {
    output: Vec<u8>
}

impl Compiler {
    pub fn compile(mut self, instructions: Vec<Instruction>) -> Result<Vec<u8>, CompilerError> {
        for instruction in instructions {
            instruction.compile(&mut self)?;
        }

        Ok(self.output)
    }

    fn write_byte(&mut self, byte: u8) {
        self.output.push(byte);
    }

    fn write_word(&mut self, word: u32) {
        let word = word.to_le_bytes();
        self.output.extend(&word);
    }
}
