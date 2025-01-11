use thiserror::Error;

pub type Result<T> = std::result::Result<T, RabcError>;

#[derive(Debug, Error)]
pub enum RabcError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("utf8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("Invalid compression: {0}")]
    InvalidCompression(char),
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),
    #[error("Invalid Deflate Stream: {0}")]
    InvalidDeflateStream(String),
    #[error("Invalid LZMA Stream: {0}")]
    InvalidLzmaStream(String),
    #[error("Unsupported compression: {0}")]
    UnsupportedCompression(&'static str),
    #[error("Invalid namespace type: {0}")]
    InvalidNamespaceType(u32),
    #[error("Invalid multiname kind: {0}")]
    InvalidMultinameKind(u8),
    #[error("Invalid method out of bound: {0}")]
    MethodOutOfBound(u32),
    #[error("Invalid method kind: {0}")]
    InvalidTraitKind(u8),
    #[error("Invalid opcode: {0}")]
    InvalidOpCode(u8),
    #[error("Index out of bounds in {0}: {1} > {2}")]
    IndexOutOfBounds(&'static str, usize, usize),

    #[cfg(feature = "lzma-rs")]
    #[error("Lzma error: {0}")]
    LzmaError(#[from] lzma_rs::error::Error),
}
