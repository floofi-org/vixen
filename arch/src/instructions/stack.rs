use crate::core::instruction::Addressing;
use crate::core::Interrupt;
use crate::core::MemoryCell;
use crate::core::Operand;
use crate::core::registers::StatusRegister;
use crate::CPU;
use crate::cpu::user_stack::UserStack;
use crate::InstructionResult;

pub fn pha(mode: &[Addressing; 3], _operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode[0] {
        cpu.user_stack_push_word(cpu.registers.r0)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn pla(mode: &[Addressing; 3], _operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode[0] {
        cpu.registers.r0 = cpu.user_stack_pull_word()?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn phx(mode: &[Addressing; 3], _operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode[0] {
        cpu.user_stack_push_word(cpu.registers.r1)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn plx(mode: &[Addressing; 3], _operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode[0] {
        cpu.registers.r1 = cpu.user_stack_pull_word()?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn phy(mode: &[Addressing; 3], _operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode[0] {
        cpu.user_stack_push_word(cpu.registers.r2)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn ply(mode: &[Addressing; 3], _operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode[0] {
        cpu.registers.r2 = cpu.user_stack_pull_word()?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn psh(mode: &[Addressing; 3], operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Immediate | Addressing::Direct | Addressing::Absolute = mode[0] {
        cpu.user_stack_push_word(operands[0].read_word()?)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn pll(mode: &[Addressing; 3], operands: &mut [Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct | Addressing::Absolute = mode[0] {
        let word = cpu.user_stack_pull_word()?;
        operands[0].write_word(cpu, word)?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn php(mode: &[Addressing; 3], _operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode[0] {
        let sr: u8 = cpu.status_register.into();
        cpu.user_stack_push_word(u32::from(sr))?;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn plp(mode: &[Addressing; 3], _operands: &[Operand; 3], cpu: &mut CPU) -> InstructionResult {
    // Status register is always 8-bit
    #[allow(clippy::cast_possible_truncation)]
    if let Addressing::Implied = mode[0] {
        let word = cpu.user_stack_pull_word()?;
        cpu.status_register = StatusRegister::from(word as u8);
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}
