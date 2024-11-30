use alloc::vec::Vec;

pub struct Specification<'a> {
    pub name: &'a str,
    pub id: u16,
    pub microarchitecture: &'a str,
    pub microarchitecture_name: &'a str,
    pub data_width: u8,
    pub address_width: u8,
    pub max_ram: u16,
    pub microcode: u16
}

impl From<Specification<'_>> for Vec<u8> {
    fn from(value: Specification) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(511);

        let name_bytes = value.name.as_bytes();
        let id_bytes = value.id.to_le_bytes();
        let microarchitecture_bytes = value.microarchitecture.as_bytes();
        let microarchitecture_name_bytes = value.microarchitecture_name.as_bytes();
        let data_width = value.data_width;
        let address_width = value.address_width;
        let max_ram_bytes = value.max_ram.to_le_bytes();
        let microcode_bytes = value.microcode.to_le_bytes();

        bytes.push(name_bytes.len() as u8);
        bytes.extend_from_slice(name_bytes);
        bytes.extend_from_slice(&id_bytes);
        bytes.push(microarchitecture_bytes.len() as u8);
        bytes.extend_from_slice(microarchitecture_bytes);
        bytes.push(microarchitecture_name_bytes.len() as u8);
        bytes.extend_from_slice(microarchitecture_name_bytes);
        bytes.push(data_width);
        bytes.push(address_width);
        bytes.extend_from_slice(&max_ram_bytes);
        bytes.extend_from_slice(&microcode_bytes);

        bytes
    }
}