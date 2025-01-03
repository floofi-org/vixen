use core::fmt::{Display, Formatter};

pub struct ExtractedBinaryData<'a>(pub &'a [u8]);

impl Display for ExtractedBinaryData<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let second_to_last = self.0.len() - 1;
        let bytes = &self.0[..second_to_last];

        for byte in bytes {
            write!(f, "{byte:0>2x} ")?;
        }

        write!(f, "{:0>2x}", self.0.last().unwrap())
    }
}
