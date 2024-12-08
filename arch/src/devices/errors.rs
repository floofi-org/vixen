pub type BusResult<T> = Result<T, BusError>;

#[repr(u32)]
pub enum BusError {
    PortOutOfRange,
    ReadOnly,
    WriteOnly,
    DeviceEvent,
    EmptyBuffer
}
