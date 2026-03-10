use bytes::{Buf, BufMut};

use crate::{errors::ProtocolErrors, traits::{Decode, Encode, Message}};

#[derive(Debug)]
pub struct NewSymbol {
    pub symbol_id: u32,
    pub ticker: [u8; 16],
}

impl Message for NewSymbol {
    const MSG_TYPE: u8 = 108;
    const MSG_SIZE: usize = 20;
}

impl Encode for NewSymbol {
    fn encode(&self, buf: &mut bytes::BytesMut) {
        buf.put_u32(self.symbol_id);
        let _ = self.ticker.map(|byte| buf.put_u8(byte));
    }
}

impl Decode for NewSymbol {
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, crate::errors::ProtocolErrors> {
        if buf.len() != Self::MSG_SIZE {
            return Err(ProtocolErrors::InvalidMessageLength);
        };

        let symbol_id = buf.get_u32();
        let mut ticker: [u8; 16] = [0u8; 16];

        for (i, byte) in buf[..16].iter().enumerate() {
            ticker[i] = *byte;
        }

        return Ok( Self {
            symbol_id,
            ticker,
        });
    }
}
