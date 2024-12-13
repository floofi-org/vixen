use crate::models::Operand;

use super::{Compile, Compiler, CompilerError};

impl Compile for Operand {
    fn compile(self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        compiler.write_word(self.into());
        Ok(())
    }
}
