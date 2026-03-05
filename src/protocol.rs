use std::{io::Write, string::ParseError};

use crate::errors::{self, ProtocolErrors};

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

impl TryFrom<u8> for MessageType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, String> {
        match value {
            1 => Ok(Self::CreateLimitOrder),
            2 => Ok(Self::CreateMarketOrder),
            3 => Ok(Self::CancelOrder),
            4 => Ok(Self::Error),
            _ => Err(String::from("Invalid value")),
        }
    }
}

impl Message {
    pub fn from(msg_type: MessageType, bytes: &[u8]) -> Result<Self, ProtocolErrors> {
        if bytes.len() <= 1 {
            return Err(ProtocolErrors::InvalidBody);
        }

        match msg_type {
            MessageType::CreateLimitOrder => {
                return Message::parse_limit_order(&bytes[0..]);
            }
            _ => todo!()
        }
    }

    fn parse_limit_order(bytes: &[u8]) -> Result<Message, ProtocolErrors> {
        let symbol = u32::from_le_bytes(bytes[..4].try_into()?);
        let side = bytes[4];
        let price = u32::from_le_bytes(bytes[5..9].try_into()?);
        let quantity = u32::from_le_bytes(bytes[9..13].try_into()?);

        return Ok(Message::CreateLimitOrder { symbol, side, price, quantity });
    }

    pub fn as_byte(&self) -> Vec<u8> {
        let mut message: Vec<u8> = Vec::new();

        match self {
            Self::Error(err) => {
                let error_code: &[u8; 1] = &[err.get_error_code()];
                let _ = message.write(error_code);
            },
            Self::CreateLimitOrder { symbol, side, price, quantity } => {
                let mut buf = Vec::new();
                let _ = buf.write_all(&symbol.to_le_bytes());
                let _ = buf.write_all(&side.to_le_bytes());
                let _ = buf.write_all(&price.to_le_bytes());
                let _ = buf.write_all(&quantity.to_le_bytes());

                return buf;
            }
            _ => {}
        };

        return message;
    }
}
