use crate::core::instruction::Addressing;
use crate::core::Interrupt;
use crate::core::MemoryCell;
use crate::core::Operand;
use crate::core::registers::StatusRegister;
use crate::CPU;
use crate::cpu::user_stack::UserStack;
use crate::InstructionResult;

pub fn pha(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.user_stack_push_word(cpu.registers.r0)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn pla(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.registers.r0 = cpu.user_stack_pull_word()?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn phx(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.user_stack_push_word(cpu.registers.r1)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn plx(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.registers.r1 = cpu.user_stack_pull_word()?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn phy(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.user_stack_push_word(cpu.registers.r2)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn ply(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        cpu.registers.r2 = cpu.user_stack_pull_word()?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn psh(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::Direct | Addressing::Absolute = mode {
        cpu.user_stack_push_word(operands[0].read_word()?)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn pll(mode: Addressing, operands: &mut [Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct | Addressing::Absolute = mode {
        let word = cpu.user_stack_pull_word()?;
        operands[0].write_word(cpu, word)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn php(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        let sr: u8 = cpu.status_register.into();
        cpu.user_stack_push_word(u32::from(sr))?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn plp(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    // Status register is always 8-bit
    #[allow(clippy::cast_possible_truncation)]
    if let Addressing::Implied = mode {
        let word = cpu.user_stack_pull_word()?;
        cpu.status_register = StatusRegister::from(word as u8);
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}
