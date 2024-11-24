use crate::core::instruction_mode::InstructionMode;
use crate::core::instruction_operation::InstructionOperation;
use crate::core::operand::Operand;

#[derive(Debug)]
pub struct Instruction<'a> {
    pub operation: InstructionOperation,
    pub mode: InstructionMode,
    pub operands: [Operand<'a>; 2]
}

#[macro_export]
macro_rules! isa {
    ( $value: expr, $( $x: expr => $y: ident ),+ ) => {
        {
            match $value {
            $(
                $x => Ok(Self::$y),
            )*
                _ => Err(Interrupt::IllegalInstruction)
            }
        }
    };
}