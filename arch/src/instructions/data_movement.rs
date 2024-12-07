use crate::core::instruction::Addressing;
use crate::core::Interrupt;
use crate::core::MemoryCell;
use crate::core::Operand;
use crate::CPU;
use crate::InstructionResult;

pub fn lda(mode: Addressing, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Absolute = mode {
        cpu.registers.r0 = operands[0].read_word()?;
        cpu.status_register.zero = cpu.registers.r0 == 0;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn ldx(mode: Addressing, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Absolute = mode {
        cpu.registers.r1 = operands[0].read_word()?;
        cpu.status_register.zero = cpu.registers.r1 == 0;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn ldy(mode: Addressing, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Absolute = mode {
        cpu.registers.r2 = operands[0].read_word()?;
        cpu.status_register.zero = cpu.registers.r2 == 0;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn ldz(mode: Addressing, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct = mode {
        operands[0].write_word(cpu, 0)?;
        cpu.status_register.zero = true;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn sta(mode: Addressing, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Absolute = mode {
        operands[0].write_word(cpu, cpu.registers.r0)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn stx(mode: Addressing, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Absolute = mode {
        operands[0].write_word(cpu, cpu.registers.r1)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn sty(mode: Addressing, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Absolute = mode {
        operands[0].write_word(cpu, cpu.registers.r2)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn mov(mode: Addressing, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct | Addressing::Absolute = mode {
        let value = operands[0].read_word()?;
        operands[1].write_word(cpu, value)?;
        operands[0].write_word(cpu, 0)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn swp(mode: Addressing, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct | Addressing::Absolute = mode {
        let value1 = operands[0].read_word()?;
        let value2 = operands[1].read_word()?;
        operands[1].write_word(cpu, value1)?;
        operands[0].write_word(cpu, value2)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn clr(mode: Addressing, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Absolute = mode {
        operands[0].write_word(cpu, 0)?;
        cpu.status_register.zero = true;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn sec(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.status_register.carry = true;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn clc(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.status_register.carry = false;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn sei(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.status_register.interrupt_disable = true;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn cli(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.status_register.interrupt_disable = false;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn clv(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.status_register.overflow = false;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}