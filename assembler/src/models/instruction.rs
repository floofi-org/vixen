use vixen::core::instruction::Operation;

use super::Operand;

#[derive(Debug)]
pub struct Instruction {
    pub operation: Operation,
    pub operands: Vec<Operand>,
}
