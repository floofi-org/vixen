use crate::core::instruction::Addressing;
use crate::core::Interrupt;
use crate::core::MemoryCell;
use crate::core::Operand;
use crate::CPU;
use crate::InstructionResult;

pub fn inc(mode: &[Addressing; 3], operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Absolute | Addressing::Direct = mode[0] {
        let initial = operands[0].read_word()?;
        let result = initial.overflowing_add(1);

        cpu.status_register.carry = result.1;
        cpu.status_register.overflow = initial == 2_147_483_646;
        operands[0].write_word(cpu, result.0)?;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn dec(mode: &[Addressing; 3], operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Absolute | Addressing::Direct = mode[0] {
        let initial = operands[0].read_word()?;
        let result = initial.overflowing_sub(1);

        cpu.status_register.carry = result.1;
        cpu.status_register.overflow = initial == 2_147_483_647;
        cpu.status_register.zero = result.0 == 0;
        operands[0].write_word(cpu, result.0)?;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}
