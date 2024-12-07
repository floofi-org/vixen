use crate::core::instruction::Addressing;
use crate::core::Interrupt;
use crate::core::MemoryCell;
use crate::core::Operand;
use crate::CPU;
use crate::cpu::SystemStack;
use crate::InstructionResult;

pub fn jmp(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Absolute | Addressing::Relative = mode {
        let position = &operands[0].read_word()?;
        cpu.program_counter = position - 10;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn jsr(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Absolute = mode {
        let position = &operands[0].read_word()?;
        cpu.system_stack_save_state()?;
        cpu.program_counter = position - 10;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn ret(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        let position = cpu.system_stack_pull_word()?;
        cpu.program_counter = position - 10;
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn beq(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Relative = mode {
        if cpu.status_register.zero {
            jmp(mode, operands, cpu)
        } else {
            Ok(())
        }
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn bne(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Relative = mode {
        if cpu.status_register.zero {
            Ok(())
        } else {
            jmp(mode, operands, cpu)
        }
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn bec(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Relative = mode {
        if cpu.status_register.carry {
            jmp(mode, operands, cpu)
        } else {
            Ok(())
        }
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn bnc(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Relative = mode {
        if cpu.status_register.carry {
            Ok(())
        } else {
            jmp(mode, operands, cpu)
        }
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn beo(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Relative = mode {
        if cpu.status_register.overflow {
            jmp(mode, operands, cpu)
        } else {
            Ok(())
        }
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn bno(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Relative = mode {
        if cpu.status_register.overflow {
            Ok(())
        } else {
            jmp(mode, operands, cpu)
        }
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn int(mode: Addressing, _operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        // They literally are just 8-bit binary numbers
        #[allow(clippy::unreadable_literal)]
        Err(match cpu.registers.r0 & 0b00001111 {
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

pub fn irt(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Direct = mode {
        cpu.status_register.interrupt = false;
        cpu.status_register.double_fault = false;
        ret(mode, operands, cpu)
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn nop(mode: Addressing, _operands: &[Operand; 2], _cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        Ok(())
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn jam(mode: Addressing, _operands: &[Operand; 2], _cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Implied = mode {
        #[allow(clippy::empty_loop)]
        loop {}
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn bpl(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Relative = mode {
        if cpu.status_register.negative {
            jmp(mode, operands, cpu)
        } else {
            Ok(())
        }
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}

pub fn bmi(mode: Addressing, operands: &[Operand; 2], cpu: &mut CPU) -> InstructionResult {
    if let Addressing::Relative = mode {
        if cpu.status_register.negative {
            jmp(mode, operands, cpu)
        } else {
            Ok(())
        }
    } else {
        Err(Interrupt::IllegalInstruction)
    }
}