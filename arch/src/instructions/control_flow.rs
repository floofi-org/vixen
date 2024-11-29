use crate::core::instruction::instruction_mode::InstructionMode;
use crate::core::interrupt::Interrupt;
use crate::core::memory_cell::MemoryCell;
use crate::core::operand::Operand;
use crate::cpu::CPU;
use crate::cpu::stack::SystemStack;
use crate::InstructionResult;

pub fn jmp(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Absolute | InstructionMode::Relative = mode {
        let position = &operands[0].read_dword()?;
        cpu.program_counter = position - 6;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn jsr(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Absolute = mode {
        let position = &operands[0].read_dword()?;
        cpu.system_stack_save_state()?;
        cpu.program_counter = position - 6;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn ret(mode: InstructionMode, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Implied = mode {
        let position = cpu.system_stack_pull_dword()?;
        cpu.program_counter = position - 6;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn beq(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Relative = mode {
        if cpu.status_register.zero {
            jmp(mode, operands, cpu)
        } else {
            Ok(())
        }
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn bne(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Relative = mode {
        if !cpu.status_register.zero {
            jmp(mode, operands, cpu)
        } else {
            Ok(())
        }
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn bec(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Relative = mode {
        if cpu.status_register.carry {
            jmp(mode, operands, cpu)
        } else {
            Ok(())
        }
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn bnc(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Relative = mode {
        if !cpu.status_register.carry {
            jmp(mode, operands, cpu)
        } else {
            Ok(())
        }
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn beo(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Relative = mode {
        if cpu.status_register.overflow {
            jmp(mode, operands, cpu)
        } else {
            Ok(())
        }
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn bno(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Relative = mode {
        if !cpu.status_register.overflow {
            jmp(mode, operands, cpu)
        } else {
            Ok(())
        }
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn int(mode: InstructionMode, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Implied = mode {
        Err(match cpu.registers.a & 0b00001111 {
            0x0 => Interrupt::User1,
            0x1 => Interrupt::User2,
            0x2 => Interrupt::User3,
            0x3 => Interrupt::User4,
            0x4 => Interrupt::User5,
            0x5 => Interrupt::User6,
            0x6 => Interrupt::User7,
            0x7 => Interrupt::User8,
            0x8 => Interrupt::User9,
            0x9 => Interrupt::User10,
            0xA => Interrupt::User11,
            0xB => Interrupt::User12,
            0xC => Interrupt::User13,
            0xD => Interrupt::User14,
            0xE => Interrupt::User15,
            _ => Interrupt::User16
        })
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn irt(mode: InstructionMode, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Implied = mode {
        cpu.status_register.interrupt = false;
        cpu.status_register.double_fault = false;
        ret(mode, operands, cpu)
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn nop(mode: InstructionMode, _operands: &[Operand; 2], _cpu: &mut CPU) -> InstructionResult {
    if let InstructionMode::Implied = mode {
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}