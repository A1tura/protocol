pub const HEADER_SIZE: usize = 11;

pub enum HeaderError {
    InvalidLength = 1,
}

pub struct Header {
    pub version: u32,
    pub length: u16,
    pub seq_num: u32,
    pub msg_type: u8,
}

impl Header {
    pub fn from(header: &[u8]) -> Result<Self, HeaderError> {
        if header.len() != HEADER_SIZE {
            return Err(HeaderError::InvalidLength);
        }

        let version = u32::from_le_bytes(header[..=4].try_into().unwrap());
        let length = u16::from_le_bytes(header[4..=6].try_into().unwrap());
        let seq_num = u32::from_le_bytes(header[6..=10].try_into().unwrap());
        let msg_type = u8::from_le_bytes(header[10..=11].try_into().unwrap());

        return Ok(Self {
            version,
            length,
            seq_num,
            msg_type,
        });
    }
}
