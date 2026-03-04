use super::header::HeaderError;

pub enum ProtocolErrors {
    HeaderError(HeaderError),
}
