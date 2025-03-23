use crate::{
    error::{RabcError, Result},
    StreamReader, StreamWriter,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum NamespaceKind {
    #[default]
    Star = 0x0,
    Namespace = 0x08,
    Package = 0x16,
    PackageInternal = 0x17,
    Protected = 0x18,
    Explicit = 0x19,
    StaticProtected = 0x1A,
    Private = 0x05,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Namespace {
    pub kind: NamespaceKind,
    pub name: u32,
}

impl Namespace {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        Ok(Self {
            kind: NamespaceKind::from(stream.read_u30()?)?,
            name: stream.read_u30()?,
        })
    }

    pub fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30((&self.kind).into())?;
        stream.write_u30(self.name)?;
        Ok(())
    }
}

impl NamespaceKind {
    pub fn from(id: u32) -> Result<Self> {
        match id {
            0x00 => Ok(Self::Star),
            0x08 => Ok(Self::Namespace),
            0x16 => Ok(Self::Package),
            0x17 => Ok(Self::PackageInternal),
            0x18 => Ok(Self::Protected),
            0x19 => Ok(Self::Explicit),
            0x1A => Ok(Self::StaticProtected),
            0x05 => Ok(Self::Private),
            _ => Err(RabcError::InvalidNamespaceType(id)),
        }
    }
}

impl From<&NamespaceKind> for u32 {
    fn from(value: &NamespaceKind) -> Self {
        match value {
            NamespaceKind::Star => 0x00,
            NamespaceKind::Namespace => 0x08,
            NamespaceKind::Package => 0x16,
            NamespaceKind::PackageInternal => 0x17,
            NamespaceKind::Protected => 0x18,
            NamespaceKind::Explicit => 0x19,
            NamespaceKind::StaticProtected => 0x1A,
            NamespaceKind::Private => 0x05,
        }
    }
}
