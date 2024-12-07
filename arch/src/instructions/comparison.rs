use crate::core::instruction::Addressing;
use crate::core::Interrupt;
use crate::core::MemoryCell;
use crate::core::Operand;
use crate::CPU;
use crate::InstructionResult;

pub fn cmp(mode: Addressing, operand: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::Direct | Addressing::Absolute = mode {
        let value1 = cpu.registers.a;
        let value2 = operand[0].read_word()?;

        cpu.status_register.zero = value1 == value2;
        cpu.status_register.carry = value1 >= value2;
        cpu.status_register.negative = value1 < value2;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn cpx(mode: Addressing, operand: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::Direct | Addressing::Absolute = mode {
        let value1 = cpu.registers.x;
        let value2 = operand[0].read_word()?;

        cpu.status_register.zero = value1 == value2;
        cpu.status_register.carry = value1 >= value2;
        cpu.status_register.negative = value1 < value2;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn cpy(mode: Addressing, operand: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::Direct | Addressing::Absolute = mode {
        let value1 = cpu.registers.y;
        let value2 = operand[0].read_word()?;

        cpu.status_register.zero = value1 == value2;
        cpu.status_register.carry = value1 >= value2;
        cpu.status_register.negative = value1 < value2;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn lte(mode: Addressing, operand: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::Direct | Addressing::Absolute = mode {
        let value1 = cpu.registers.a;
        let value2 = operand[0].read_word()?;

        cpu.status_register.zero = value1 <= value2;
        cpu.status_register.carry = value1 > value2;
        cpu.status_register.negative = value1 < value2;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn gte(mode: Addressing, operand: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::Direct | Addressing::Absolute = mode {
        let value1 = cpu.registers.a;
        let value2 = operand[0].read_word()?;

        cpu.status_register.zero = value1 >= value2;
        cpu.status_register.carry = value1 < value2;
        cpu.status_register.negative = value1 < value2;

        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn srz(mode: Addressing, operand: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct = mode {
        operand[0].write_word(cpu, u32::from(cpu.status_register.zero))?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn src(mode: Addressing, operand: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct = mode {
        operand[0].write_word(cpu, u32::from(cpu.status_register.carry))?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn sro(mode: Addressing, operand: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct = mode {
        operand[0].write_word(cpu, u32::from(cpu.status_register.overflow))?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}
