use crate::core::Interrupt;
use crate::core::registers::StatusRegister;
use crate::CPU;
use crate::CPUResult;

pub trait SystemStack {
    fn system_stack_push_word(&mut self, value: u8) -> CPUResult<()>;
    fn system_stack_push_dword(&mut self, value: u16) -> CPUResult<()>;
    fn system_stack_pull_word(&mut self) -> CPUResult<u8>;
    fn system_stack_pull_dword(&mut self) -> CPUResult<u16>;
    fn system_stack_save_state(&mut self) -> CPUResult<()>;
    fn system_stack_restore_state(&mut self) -> CPUResult<()>;
    fn system_stack_pull_state(&mut self) -> CPUResult<(u16, StatusRegister)>;
}

impl SystemStack for CPU {
    fn system_stack_push_word(&mut self, value: u8) -> CPUResult<()> {
        if self.system_stack.len() > 256 {
            Err(Interrupt::StackOverflow)
        } else {
            self.system_stack.push(u16::from(value));
            Ok(())
        }
    }

    fn system_stack_push_dword(&mut self, value: u16) -> CPUResult<()> {
        if self.system_stack.len() > 256 {
            Err(Interrupt::StackOverflow)
        } else {
            self.system_stack.push(value);
            Ok(())
        }
    }

    fn system_stack_pull_word(&mut self) -> CPUResult<u8> {
        if self.system_stack.is_empty() {
            Err(Interrupt::StackUnderflow)
        } else {
            #[allow(clippy::cast_possible_truncation)]
            // This is intended as the system stack can store both u16 and u8 values
            match self.system_stack.pop().ok_or(Interrupt::IllegalMemory) {
                Ok(v) => Ok(v as u8),
                Err(i) => Err(i)
            }
        }
    }

    fn system_stack_pull_dword(&mut self) -> CPUResult<u16> {
        if self.system_stack.is_empty() {
            Err(Interrupt::StackUnderflow)
        } else {
            self.system_stack.pop().ok_or(Interrupt::IllegalMemory)
        }
    }

    fn system_stack_save_state(&mut self) -> CPUResult<()> {
        self.system_stack_push_word(self.status_register.into())?;
        self.system_stack_push_dword(self.program_counter)
    }

    fn system_stack_restore_state(&mut self) -> CPUResult<()> {
        self.program_counter = self.system_stack_pull_dword()?;
        self.status_register = StatusRegister::from(self.system_stack_pull_word()?);
        Ok(())
    }

    fn system_stack_pull_state(&mut self) -> CPUResult<(u16, StatusRegister)> {
        let program_counter = self.system_stack_pull_dword()?;
        let status_register = StatusRegister::from(self.system_stack_pull_word()?);
        Ok((program_counter, status_register))
    }
}
