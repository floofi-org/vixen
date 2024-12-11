use crate::core::instruction::Addressing;
use crate::core::Interrupt;
use crate::core::MemoryCell;
use crate::core::Operand;
use crate::CPU;
use crate::InstructionResult;

pub fn cmp(mode: &[Addressing; 3], operand: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::Direct | Addressing::Absolute = mode[0] {
        let value1 = cpu.registers.r0;
        let value2 = operand[0].read_word()?;

        cpu.status_register.zero = value1 == value2;
        cpu.status_register.carry = value1 >= value2;
        cpu.status_register.negative = value1 < value2;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn lte(mode: &[Addressing; 3], operand: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::Direct | Addressing::Absolute = mode[0] {
        let value1 = cpu.registers.r0;
        let value2 = operand[0].read_word()?;

        cpu.status_register.zero = value1 <= value2;
        cpu.status_register.carry = value1 > value2;
        cpu.status_register.negative = value1 < value2;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn gte(mode: &[Addressing; 3], operand: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::Direct | Addressing::Absolute = mode[0] {
        let value1 = cpu.registers.r0;
        let value2 = operand[0].read_word()?;

        cpu.status_register.zero = value1 >= value2;
        cpu.status_register.carry = value1 < value2;
        cpu.status_register.negative = value1 < value2;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn srz(mode: &[Addressing; 3], operand: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct = mode[0] {
        operand[0].write_word(cpu, u32::from(cpu.status_register.zero))?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn src(mode: &[Addressing; 3], operand: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct = mode[0] {
        operand[0].write_word(cpu, u32::from(cpu.status_register.carry))?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn sro(mode: &[Addressing; 3], operand: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct = mode[0] {
        operand[0].write_word(cpu, u32::from(cpu.status_register.overflow))?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}
