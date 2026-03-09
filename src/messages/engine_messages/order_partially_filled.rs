use bytes::{Buf, BufMut};

use crate::{
    errors::ProtocolErrors,
    traits::{Decode, Encode, Message},
};

#[derive(Debug)]
pub struct OrderPartiallyFilled {
    pub symbol_id: u32,
    pub order_id: u32,
    pub remaining: u32,
}

impl Message for OrderPartiallyFilled {
    const MSG_TYPE: u8 = 103;
    const MSG_SIZE: usize = 12;
}

impl Encode for OrderPartiallyFilled {
    fn encode(&self, buf: &mut bytes::BytesMut) {
        buf.put_u32(self.symbol_id);
        buf.put_u32(self.order_id);
        buf.put_u32(self.remaining);
    }
}

impl Decode for OrderPartiallyFilled {
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, crate::errors::ProtocolErrors> {
        if buf.len() != Self::MSG_SIZE {
            return Err(ProtocolErrors::InvalidMessageLength);
        };

        return Ok(Self {
            symbol_id: buf.get_u32(),
            order_id: buf.get_u32(),
            remaining: buf.get_u32(),
        });
    }
}
