use crate::core::Interrupt;
use crate::core::registers::StatusRegister;
use crate::CPU;
use crate::CPUResult;

pub trait UserStack {
    fn user_stack_push_word(&mut self, value: u8) -> CPUResult<()>;
    fn user_stack_push_dword(&mut self, value: u16) -> CPUResult<()>;
    fn user_stack_pull_word(&mut self) -> CPUResult<u8>;
    fn user_stack_pull_dword(&mut self) -> CPUResult<u16>;
    fn user_stack_save_state(&mut self) -> CPUResult<()>;
    fn user_stack_restore_state(&mut self) -> CPUResult<()>;
    fn user_stack_pull_state(&mut self) -> CPUResult<(u16, StatusRegister)>;
}

impl UserStack for CPU {
    fn user_stack_push_word(&mut self, value: u8) -> CPUResult<()> {
        if self.stack_pointer <= 0x0100 {
            Err(Interrupt::StackOverflow)
        } else {
            self.memory[self.stack_pointer as usize] = value;
            self.stack_pointer -= 1;
            Ok(())
        }
    }

    fn user_stack_push_dword(&mut self, value: u16) -> CPUResult<()> {
        if self.stack_pointer <= 0x0101 {
            Err(Interrupt::StackOverflow)
        } else {
            self.memory[self.stack_pointer as usize] = (value & 0xFF) as u8;
            self.memory[(self.stack_pointer - 1) as usize] = (value >> 8) as u8;
            self.stack_pointer -= 2;
            Ok(())
        }
    }

    fn user_stack_pull_word(&mut self) -> CPUResult<u8> {
        if self.stack_pointer >= 0x01FF {
            Err(Interrupt::StackUnderflow)
        } else {
            self.stack_pointer += 1;
            Ok(self.memory[(self.stack_pointer - 1) as usize])
        }
    }

    fn user_stack_pull_dword(&mut self) -> CPUResult<u16> {
        if self.stack_pointer >= 0x01FE {
            Err(Interrupt::StackUnderflow)
        } else {
            self.stack_pointer += 2;
            Ok(u16::from(self.memory[(self.stack_pointer + 2) as usize]) * 0x100 + u16::from(self.memory[(self.stack_pointer + 1) as usize]))
        }
    }

    fn user_stack_save_state(&mut self) -> CPUResult<()> {
        self.user_stack_push_word(self.status_register.into())?;
        self.user_stack_push_dword(self.program_counter)
    }

    fn user_stack_restore_state(&mut self) -> CPUResult<()> {
        self.program_counter = self.user_stack_pull_dword()?;
        self.status_register = StatusRegister::from(self.user_stack_pull_word()?);
        Ok(())
    }

    fn user_stack_pull_state(&mut self) -> CPUResult<(u16, StatusRegister)> {
        let program_counter = self.user_stack_pull_dword()?;
        let status_register = StatusRegister::from(self.user_stack_pull_word()?);
        Ok((program_counter, status_register))
    }
}
