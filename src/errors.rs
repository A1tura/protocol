use super::header::HeaderError;

pub enum ProtocolErrors {
    HeaderError(HeaderError),
}

impl ProtocolErrors {
    pub fn get_error_code(&self) -> u8 {
        match self {
            Self::HeaderError(err) => {
                match err {
                    HeaderError::InvalidLength => {
                        return 1;
                    },
                    HeaderError::InvalidHeader => {
                        return 2
                    }
                }
            }
        }
    }
}
