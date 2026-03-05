use super::header::HeaderError;

#[derive(Debug)]
pub enum ProtocolErrors {
    HeaderError(HeaderError),
    InvalidBody,
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
            },
            Self::InvalidBody => return 3,
        }
    }
}

impl From<std::array::TryFromSliceError> for ProtocolErrors {
    fn from(_: std::array::TryFromSliceError) -> Self {
        Self::InvalidBody
    }
}
