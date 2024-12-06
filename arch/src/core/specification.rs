use alloc::vec::Vec;

pub struct Specification<'a> {
    pub name: &'a [u8],
    pub id: u16,
    pub microarchitecture: &'a [u8],
    pub microarchitecture_name: &'a [u8],
    pub data_width: u8,
    pub address_width: u8,
    pub max_ram: u16,
    pub microcode: u16
}

impl From<Specification<'_>> for Vec<u8> {
    #[allow(clippy::cast_possible_truncation)]
    // Strings in the specification are truncated to max 255 chars
    fn from(value: Specification) -> Self {
        let mut bytes: Vec<u8> = Vec::with_capacity(511);

        let name_bytes = value.name;
        let id_bytes = value.id.to_le_bytes();
        let microarchitecture_bytes = value.microarchitecture;
        let microarchitecture_name_bytes = value.microarchitecture_name;
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
