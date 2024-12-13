use crate::core::instruction::Addressing;
use crate::core::Interrupt;
use crate::core::MemoryCell;
use crate::core::Operand;
use crate::CPU;
use crate::InstructionResult;

pub fn ldr(mode: &[Addressing; 3], operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Absolute = mode[0] {
        cpu.registers.r0 = operands[0].read_word()?;
        cpu.status_register.zero = cpu.registers.r0 == 0;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn str(mode: &[Addressing; 3], operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Absolute = mode[0] {
        operands[0].write_word(cpu, cpu.registers.r0)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalMemory)
    }
}

pub fn mov(mode: &[Addressing; 3], operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if (mode[0] == Addressing::Direct || mode[0] == Addressing::Absolute) &&
        (mode[1] == Addressing::Direct || mode[1] == Addressing::Absolute) {
        let value = operands[1].read_word()?;
        operands[0].write_word(cpu, value)?;
        operands[1].write_word(cpu, 0)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn swp(mode: &[Addressing; 3], operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if (mode[0] == Addressing::Direct || mode[0] == Addressing::Absolute) &&
        (mode[1] == Addressing::Direct || mode[1] == Addressing::Absolute) {
        let value1 = operands[0].read_word()?;
        let value2 = operands[1].read_word()?;
        operands[1].write_word(cpu, value1)?;
        operands[0].write_word(cpu, value2)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn clr(mode: &[Addressing; 3], operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if (mode[0] == Addressing::Direct || mode[0] == Addressing::Absolute) &&
        (mode[1] == Addressing::Direct || mode[1] == Addressing::Absolute) {
        operands[0].write_word(cpu, 0)?;
        cpu.status_register.zero = true;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn sec(mode: &[Addressing; 3], _operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode[0] {
        cpu.status_register.carry = true;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn clc(mode: &[Addressing; 3], _operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode[0] {
        cpu.status_register.carry = false;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn sei(mode: &[Addressing; 3], _operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode[0] {
        cpu.status_register.interrupt_disable = true;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn cli(mode: &[Addressing; 3], _operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode[0] {
        cpu.status_register.interrupt_disable = false;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn clv(mode: &[Addressing; 3], _operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode[0] {
        cpu.status_register.overflow = false;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}
