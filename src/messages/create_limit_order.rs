use bytes::{Buf, BufMut, BytesMut};

use crate::{errors::ProtocolErrors, traits::{Decode, Encode, Message}};

#[derive(Debug)]
pub struct CreateLimitOrder {
    pub symbol: u32,
    pub side: u8,
    pub price: u32,
    pub quantity: u32,
}

impl Message for CreateLimitOrder {
    const MSG_TYPE: u8 = 1;
    const MSG_SIZE: usize = 13;
}

impl Encode for CreateLimitOrder {
    fn encode(&self, buf: &mut bytes::BytesMut) {
        buf.put_u32(self.symbol);
        buf.put_u8(self.side);
        buf.put_u32(self.price);
        buf.put_u32(self.quantity);
    }
}

impl Decode for CreateLimitOrder {
    fn decode(buf: &mut BytesMut) -> Result<Self, ProtocolErrors> {

        if buf.len() != Self::MSG_SIZE { return Err(ProtocolErrors::InvalidMessageLength) };

        let symbol = buf.get_u32();
        let side = buf.get_u8();
        let price = buf.get_u32();
        let quantity = buf.get_u32();

        return Ok(Self {
            symbol,
            price,
            side,
            quantity
        })
    }
}
