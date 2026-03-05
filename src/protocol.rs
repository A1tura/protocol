use std::io::Write;

use crate::errors;

#[derive(Debug)]
pub enum Message {
    CreateLimitOrder {
        symbol: u32,
        side: u8,
        price: u32,
        quantity: u32,
    },
    CreateMarketOrder {
        symbol: u32,
        side: u8,
        quantity: u32,
    },
    CancelOrder {
        symbol: u32,
        order_id: u32,
    },
    Error(errors::ProtocolErrors),
}

#[derive(Debug)]
pub enum MessageType {
    CreateLimitOrder = 1,
    CreateMarketOrder = 2,
    CancelOrder = 3,
    Error = 4,
}

impl Message {
    pub fn as_byte(&self) -> Vec<u8> {
        let mut message: Vec<u8> = Vec::new();

        match self {
            Self::Error(err) => {
                let error_code: &[u8; 1] = &[err.get_error_code()];
                let _ = message.write(error_code);
            },
            _ => {}
        };

        return message;
    }
}
