use std::{borrow, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    FromUtf8Error(std::string::FromUtf8Error),
    InvalidCompression(borrow::Cow<'static, str>),
    InvalidSignature(borrow::Cow<'static, str>),
    UnsupportedCompression(borrow::Cow<'static, str>),
    InvalidNamespaceType(u32),
    InvalidMultinameKind(u8),
    MethodOutOfBound(u32),
    InvalidTraitKind(u8),
    #[cfg(feature = "lzma-rs")]
    LzmaError(lzma_rs::error::Error),
    InvalidOpCode(u8),
    IndexOutOfBounds(borrow::Cow<'static, str>, usize, usize),
}

impl Error {
    #[inline]
    pub fn invalid_compression(message: impl Into<borrow::Cow<'static, str>>) -> Self {
        Self::InvalidCompression(message.into())
    }

    #[inline]
    pub fn invalid_signature(message: impl Into<borrow::Cow<'static, str>>) -> Self {
        Self::InvalidSignature(message.into())
    }

    #[cfg(any(not(feature = "lzma-rs"), not(feature = "flate2")))]
    #[inline]
    pub fn unsupported_compression(message: impl Into<borrow::Cow<'static, str>>) -> Self {
        Self::UnsupportedCompression(message.into())
    }

    #[inline]
    pub fn index_out_of_bounds(
        name: impl Into<borrow::Cow<'static, str>>,
        index: usize,
        length: usize,
    ) -> Self {
        Self::IndexOutOfBounds(name.into(), index, length)
    }
}
impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}
impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8Error(error)
    }
}
#[cfg(feature = "lzma-rs")]
impl From<lzma_rs::error::Error> for Error {
    fn from(error: lzma_rs::error::Error) -> Self {
        Self::LzmaError(error)
    }
}
