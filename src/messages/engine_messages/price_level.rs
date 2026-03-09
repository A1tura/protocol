use bytes::{Buf, BufMut};

use crate::{
    errors::ProtocolErrors,
    traits::{Decode, Encode, Message},
};

#[derive(Debug)]
pub struct PriceLevel {
    pub symbol_id: u32,
    pub side: u8,
    pub price: f64,
    pub quantity: u32,
}

impl Message for PriceLevel {
    const MSG_TYPE: u8 = 107;
    const MSG_SIZE: usize = 17;
}

impl Encode for PriceLevel {
    fn encode(&self, buf: &mut bytes::BytesMut) {
        buf.put_u32(self.symbol_id);
        buf.put_u8(self.side);
        buf.put_f64(self.price);
        buf.put_u32(self.quantity);
    }
}

impl Decode for PriceLevel {
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, crate::errors::ProtocolErrors> {
        if buf.len() != Self::MSG_SIZE {
            return Err(ProtocolErrors::InvalidMessageLength);
        };

        return Ok(PriceLevel {
            symbol_id: buf.get_u32(),
            side: buf.get_u8(),
            price: buf.get_f64(),
            quantity: buf.get_u32(),
        });
    }
}
