use std::fmt::{Display, Formatter};

pub struct ExtractedBinaryData<'a>(pub(crate) &'a [u8]);

impl<'a> Display for ExtractedBinaryData<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut hex = String::new();
        for (index, byte) in self.0.iter().enumerate() {
            hex.push_str(&format!("{:0>2X}", byte));
            if index % 1 == 0 {
                hex.push(' ');
            }
        }
        write!(f, "{}", hex.trim())
    }
}