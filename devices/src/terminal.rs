use std::collections::VecDeque;
use std::io;
use std::io::{Stdout, Write};
use stdin::StdinReader;
use vixen::BusDevice;
use vixen::devices::errors::{BusError, BusResult};

mod stdin;

#[derive(Debug)]
pub struct Terminal {
    stdin: StdinReader,
    stdout: Stdout,
    read_buffer: VecDeque<u8>,
    write_buffer: VecDeque<u8>,
}

impl Terminal {
    #[must_use]
    pub fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::sync_channel(0);

        let stdin = StdinReader::spawn(sender, receiver);
        let stdout = io::stdout();
        let read_buffer = VecDeque::new();
        let write_buffer = VecDeque::new();

        Self {
            stdin,
            stdout,
            read_buffer,
            write_buffer,
        }
    }

    fn read(&mut self) -> BusResult<u32> {
        match self.read_buffer.pop_front() {
            Some(ch) => Ok(u32::from(ch)),
            None => Err(BusError::EmptyBuffer)
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn write(&mut self, data: u32) -> BusResult<()> {
        #[allow(clippy::cast_possible_truncation)]
        self.write_buffer.push_back(data as u8);
        Ok(())
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new()
    }
}

impl BusDevice for Terminal {
    fn get_port_count(&self) -> u32 {
        3
    }

    fn get_base_address(&self) -> u32 {
        0x0400_0200
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
        // TODO: Handle results here
        if let Some(ch) = self.write_buffer.pop_front() {
            self.stdout.write_all(&[ch]).unwrap();
        }

        if let Some(char) = self.stdin.read() {
            self.read_buffer.push_back(char);
            return Err(BusError::DeviceEvent);
        }

        Ok(())
    }
}
