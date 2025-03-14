pub mod errors;

use core::fmt::Debug;
use crate::devices::errors::BusResult;

pub trait BusDevice: Debug {
    fn get_port_count(&self) -> u32;
    fn get_base_address(&self) -> u32;
    fn read_port(&mut self, index: u32) -> BusResult<u32>;
    fn write_port(&mut self, index: u32, data: u32) -> BusResult<()>;
    fn tick(&mut self) -> BusResult<()>;
}
