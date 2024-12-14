use alloc::vec::Vec;
use crate::BASE_SYSTEM_SIZE;

#[allow(clippy::module_name_repetitions)]
pub struct StaticSpecification<'a> {
    pub name: &'a [u8],
    pub id: u32,
    pub microarchitecture: &'a [u8],
    pub microarchitecture_name: &'a [u8],
    pub data_width: u8,
    pub address_width: u8,
    pub microcode: u32
}

pub struct Specification<'a> {
    pub specification: StaticSpecification<'a>,
    pub available_ram: u32,
    pub vm_end: u32
}

impl<'a> Specification<'a> {
    #[must_use]
    pub fn new(static_specification: StaticSpecification<'a>, memory: usize) -> Specification<'a> {
        #[allow(clippy::cast_possible_truncation)]
        Self {
            specification: static_specification,
            available_ram: (memory - BASE_SYSTEM_SIZE) as u32,
            vm_end: memory as u32
        }
    }
}

impl From<Specification<'_>> for Vec<u8> {
    // Strings in the specification are truncated to max 255 chars
    #[allow(clippy::cast_possible_truncation)]
    fn from(value: Specification) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(511);

        let name_bytes = value.specification.name;
        let id_bytes = value.specification.id.to_le_bytes();
        let microarchitecture_bytes = value.specification.microarchitecture;
        let microarchitecture_name_bytes = value.specification.microarchitecture_name;
        let data_width = value.specification.data_width;
        let address_width = value.specification.address_width;
        let microcode_bytes = value.specification.microcode.to_le_bytes();
        let available_ram_bytes = value.available_ram.to_le_bytes();
        let vm_end_bytes = value.vm_end.to_le_bytes();

        bytes.push(name_bytes.len() as u8);
        bytes.extend_from_slice(name_bytes);
        bytes.extend_from_slice(&id_bytes);
        bytes.push(microarchitecture_bytes.len() as u8);
        bytes.extend_from_slice(microarchitecture_bytes);
        bytes.push(microarchitecture_name_bytes.len() as u8);
        bytes.extend_from_slice(microarchitecture_name_bytes);
        bytes.push(data_width);
        bytes.push(address_width);
        bytes.extend_from_slice(&available_ram_bytes);
        bytes.extend_from_slice(&microcode_bytes);
        bytes.extend_from_slice(&vm_end_bytes);

        bytes
    }
}
