use crate::core::Interrupt;

pub type BusResult<T> = Result<T, BusError>;

#[repr(u32)]
pub enum BusError {
    PortOutOfRange,
    ReadOnly,
    WriteOnly,
    DeviceEvent,
    EmptyBuffer,
    InternalSystem
}

impl From<Interrupt> for BusError {
    fn from(_: Interrupt) -> Self {
        Self::InternalSystem
    }
}
