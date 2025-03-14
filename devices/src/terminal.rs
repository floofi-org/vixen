use std::collections::VecDeque;
use std::io;
use std::io::{Stdout, Write};
use stdin::TerminalStdin;
use vixen::BusDevice;
use vixen::devices::errors::{BusError, BusResult};

mod stdin;

pub use stdin::StdinReader;

#[derive(Debug)]
pub struct Terminal<S: StdinReader> {
    stdin: S,
    stdout: Stdout,
    read_buffer: VecDeque<u8>,
    write_buffer: VecDeque<u8>,
}

impl Default for Terminal<TerminalStdin> {
    fn default() -> Self {
        let (sender, receiver) = std::sync::mpsc::sync_channel(512);
        let stdin = TerminalStdin::spawn(sender, receiver);

        Self::new(stdin)
    }
}

impl<S: StdinReader> Terminal<S> {
    #[must_use]
    pub fn new(stdin: S) -> Self {
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

impl<S: StdinReader> BusDevice for Terminal<S> {
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
        if let Some(ch) = self.write_buffer.pop_front() {
            self.stdout.write_all(&[ch]).unwrap();
            self.stdout.flush().unwrap();
        }

        if let Some(char) = self.stdin.read() {
            self.read_buffer.push_back(char);
            return Err(BusError::DeviceEvent);
        }

        Ok(())
    }
}
