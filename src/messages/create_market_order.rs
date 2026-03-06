use bytes::{Buf, BufMut, BytesMut};

use crate::{errors::ProtocolErrors, traits::{Decode, Encode, Message}};

#[derive(Debug)]
pub struct CreateMarketOrder {
    pub symbol: u32,
    pub side: u8,
    pub quantity: u32,
}

impl Message for CreateMarketOrder {
    const MSG_TYPE: u8 = 2;
    const MSG_SIZE: usize = 9;
}

impl Encode for CreateMarketOrder {
    fn encode(&self, buf: &mut bytes::BytesMut) {
        buf.put_u32(self.symbol);
        buf.put_u8(self.side);
        buf.put_u32(self.quantity);
    }
}

impl Decode for CreateMarketOrder {
    fn decode(buf: &mut BytesMut) -> Result<Self, ProtocolErrors> {
        if buf.len() != Self::MSG_SIZE { return Err(ProtocolErrors::InvlaidMessageLength) };

        let symbol = buf.get_u32();
        let side = buf.get_u8();
        let quantity = buf.get_u32();

        return Ok(Self {
            symbol,
            side,
            quantity
        })
    }
}
