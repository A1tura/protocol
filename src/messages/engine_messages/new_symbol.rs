use bytes::{Buf, BufMut};

use crate::{errors::ProtocolErrors, traits::{Decode, Encode, Message}};

#[derive(Debug)]
pub struct NewSymbol {
    pub symbol_id: u32,
}

impl Message for NewSymbol {
    const MSG_TYPE: u8 = 108;
    const MSG_SIZE: usize = 4;
}

impl Encode for NewSymbol {
    fn encode(&self, buf: &mut bytes::BytesMut) {
        buf.put_u32(self.symbol_id);
    }
}

impl Decode for NewSymbol {
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, crate::errors::ProtocolErrors> {
        if buf.len() != Self::MSG_SIZE {
            return Err(ProtocolErrors::InvalidMessageLength);
        };

        return Ok(Self {
            symbol_id: buf.get_u32(),
        });
    }
}
