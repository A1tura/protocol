use bytes::{Buf, BufMut};

use crate::traits::{Decode, Encode};

#[derive(Debug)]
pub enum HeaderError {
    InvalidLength = 1,
    InvalidHeader = 2,
}

impl Encode for HeaderError {
    fn encode(&self, buf: &mut bytes::BytesMut) {
        match self {
            Self::InvalidHeader => buf.put_u8(1),
            Self::InvalidLength => buf.put_u8(2),
        }
    }
}

impl Decode for HeaderError {
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, super::ProtocolErrors> {
        let error_type = buf.get_u8();

        match error_type {
            1 => Ok(HeaderError::InvalidLength),
            2 => Ok(HeaderError::InvalidHeader),
            _ => todo!()
        }
    }
}
