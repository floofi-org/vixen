use std::collections::VecDeque;
use std::io;
use std::io::{Read, Stdin, Stdout, Write};
use vixen::BusDevice;
use vixen::devices::errors::{BusError, BusResult};

#[derive(Debug)]
pub struct TerminalDevice {
    write_buffer: VecDeque<u8>,
    read_buffer: VecDeque<u8>,
    stdin: Stdin,
    stdout: Stdout
}

impl TerminalDevice {
    pub fn new() -> Self {
        Self {
            write_buffer: VecDeque::new(),
            read_buffer: VecDeque::new(),
            stdin: io::stdin(),
            stdout: io::stdout()
        }
    }
    
    fn read(&mut self) -> BusResult<u32> {
        match self.write_buffer.pop_front() {
            Some(ch) => Ok(u32::from(ch)),
            None => Err(BusError::EmptyBuffer)
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn write(&mut self, data: u32) -> BusResult<()> {
        #[allow(clippy::cast_possible_truncation)]
        self.read_buffer.push_back(data as u8);
        Ok(())
    }
}

impl BusDevice for TerminalDevice {
    fn get_port_count(&self) -> u32 {
        3
    }
    #[allow(clippy::unreadable_literal)]
    fn get_base_address(&self) -> u32 {
        0x04000200
    }

    fn read_port(&mut self, index: u32) -> BusResult<u32> {
        match index {
            0 => Err(BusError::WriteOnly),
            1 => self.read(),
            2 => Ok(u32::from(!self.write_buffer.is_empty())),
            _ => Err(BusError::PortOutOfRange),
        }
    }

    fn write_port(&mut self, index: u32, data: u32) -> BusResult<()> {
        match index {
            0 => self.write(data),
            1 | 2 => Err(BusError::ReadOnly),
            _ => Err(BusError::PortOutOfRange),
        }
    }

    fn tick(&mut self) -> BusResult<()> {
        if let Some(ch) = self.write_buffer.pop_front() {
            let _ = self.stdout.write(&[ch]);
        }

        let mut char_buffer = [0u8; 1];
        let mut stdin = self.stdin.lock();
        if let Ok(1) = stdin.read(&mut char_buffer) {
            self.read_buffer.push_back(char_buffer[0]);
            return Err(BusError::DeviceEvent);
        }

        Ok(())
    }
}
