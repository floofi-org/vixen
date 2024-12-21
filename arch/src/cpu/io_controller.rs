use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ops::{DerefMut, Range};
use crate::{BusDevice, CPUResult};
use crate::core::Interrupt;
use crate::devices::errors::BusResult;

#[derive(Debug, Default)]
pub struct IOController {
    devices: Vec<(Range<u32>, Box<dyn BusDevice>)>
}

impl IOController {
    pub fn find_device_port(&mut self, address: u32) -> BusResult<(&mut dyn BusDevice, u32)> {
        let (range, device) = self.devices
            .iter_mut()
            .find(|(range, _)| range.contains(&address))
            .ok_or(Interrupt::IllegalMemory)?;

        let device = device.deref_mut();
        let port = (address - range.start) / 4;

        Ok((device, port))
    }

    pub fn read_bus(&mut self, address: u32) -> BusResult<u32> {
        let (device, port) = self.find_device_port(address)?;
        device.read_port(port)
    }

    pub fn write_bus(&mut self, address: u32, word: u32) -> BusResult<()> {
        let (device, port) = self.find_device_port(address)?;
        device.write_port(port, word)
    }

    pub fn tick(&mut self) -> CPUResult<()> {
        for (_, device) in &mut self.devices {
            device.tick()?;
        }
        
        Ok(())
    }

    pub fn add(&mut self, device: Box<dyn BusDevice>) -> CPUResult<()> {
        let start = device.get_base_address();
        let end = device.get_base_address() + (device.get_port_count() * 4);
        if self.find_device_port(start).is_err() {
            self.devices.push((start..end, device));
            Ok(())
        } else {
            Err(Interrupt::Hardware)
        }
    }
}
