use vixen::core::{instruction::Operation, Interrupt};

use crate::assembler::{Compile, CompilerError};

use super::Operand;

#[derive(Debug)]
pub struct Instruction {
    pub operation: Operation,
    pub operands: Vec<Operand>,
}
