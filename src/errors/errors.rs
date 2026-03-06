use bytes::{Buf, BufMut, BytesMut};

use crate::{errors::HeaderError, traits::{Decode, Encode}};

#[derive(Debug)]
pub enum ProtocolErrors {
    HeaderError(HeaderError),
    InvalidBody,
    SequenceError { expected: u32, received: u32 },
    InvalidMessageType,
    InvalidMessageLength,
}

impl Encode for ProtocolErrors {
    fn encode(&self, buf: &mut BytesMut) {
        match self {
            Self::HeaderError(err) => {
                let mut header_error = BytesMut::new();
                err.encode(&mut header_error);

                buf.put_u8(1);
                buf.put(header_error);
            },
            Self::InvalidBody => buf.put_u8(2),
            Self::SequenceError { expected, received } => {
                buf.put_u8(3);
                buf.put_u32(*expected);
                buf.put_u32(*received);
            },
            Self::InvalidMessageType => buf.put_u8(4),
            Self::InvalidMessageLength => buf.put_u8(5),
        }
    }
}

impl Decode for ProtocolErrors {
    fn decode(buf: &mut BytesMut) -> Result<Self, ProtocolErrors> {
        let error_type = buf.get_u8();

        match error_type {
            1 => {
                let header_error = HeaderError::decode(buf)?;
                return Ok(Self::HeaderError(header_error));
            },
            2 => return Ok(Self::InvalidBody),
            3 => {
                let expected = buf.get_u32();
                let received = buf.get_u32();

                return Ok(Self::SequenceError { expected, received })
            },
            4 => return Ok(Self::InvalidMessageType),
            5 => return Ok(Self::InvalidMessageLength),
            _ => todo!()
        }
    }
}

impl From<std::array::TryFromSliceError> for ProtocolErrors {
    fn from(_: std::array::TryFromSliceError) -> Self {
        Self::InvalidBody
    }
}
