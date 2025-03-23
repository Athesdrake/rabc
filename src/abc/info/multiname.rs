use crate::{
    error::{RabcError, Result},
    StreamReader, StreamWriter,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Multiname {
    QName(QName),
    QNameA(QName),
    RTQName(RTQName),
    RTQNameA(RTQName),
    RTQNameL(()),
    RTQNameLA(()),
    Multiname(Multi),
    MultinameA(Multi),
    MultinameL(MultiL),
    MultinameLA(MultiL),
    Typename(Typename),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct QName {
    pub ns: u32,
    pub name: u32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RTQName {
    pub name: u32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Multi {
    pub name: u32,
    pub ns_set: u32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MultiL {
    pub ns_set: u32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Typename {
    pub qname: u32,
    pub types: Vec<u32>,
}

impl QName {
    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        Ok(Self {
            ns: stream.read_u30()?,
            name: stream.read_u30()?,
        })
    }

    fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.ns)?;
        stream.write_u30(self.name)?;
        Ok(())
    }
}
impl RTQName {
    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        Ok(Self {
            name: stream.read_u30()?,
        })
    }

    fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.name)?;
        Ok(())
    }
}
impl Multi {
    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        Ok(Self {
            name: stream.read_u30()?,
            ns_set: stream.read_u30()?,
        })
    }

    fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.name)?;
        stream.write_u30(self.ns_set)?;
        Ok(())
    }
}
impl MultiL {
    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        Ok(Self {
            ns_set: stream.read_u30()?,
        })
    }

    fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.ns_set)?;
        Ok(())
    }
}
impl Typename {
    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        let qname = stream.read_u30()?;
        let count = stream.read_u30()?;
        let mut types = Vec::with_capacity(count as usize);
        for _ in 0..count {
            types.push(stream.read_u30()?);
        }

        Ok(Self { qname, types })
    }

    fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.qname)?;
        stream.write_u30(self.types.len() as u32)?;
        for t in &self.types {
            stream.write_u30(*t)?;
        }
        Ok(())
    }
}

impl Multiname {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn kind(&self) -> u8 {
        match self {
            Self::QName(_) => 0x07,
            Self::QNameA(_) => 0x0D,
            Self::RTQName(_) => 0x0F,
            Self::RTQNameA(_) => 0x10,
            Self::RTQNameL(_) => 0x11,
            Self::RTQNameLA(_) => 0x12,
            Self::Multiname(_) => 0x09,
            Self::MultinameA(_) => 0x0E,
            Self::MultinameL(_) => 0x1B,
            Self::MultinameLA(_) => 0x1C,
            Self::Typename(_) => 0x1D,
        }
    }

    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        match stream.read_u8()? {
            0x07 => Ok(Self::QName(QName::read(stream)?)),
            0x0D => Ok(Self::QNameA(QName::read(stream)?)),
            0x0F => Ok(Self::RTQName(RTQName::read(stream)?)),
            0x10 => Ok(Self::RTQNameA(RTQName::read(stream)?)),
            0x11 => Ok(Self::RTQNameL(())), // This kind has no associated data.
            0x12 => Ok(Self::RTQNameLA(())), // This kind has no associated data.
            0x09 => Ok(Self::Multiname(Multi::read(stream)?)),
            0x0E => Ok(Self::MultinameA(Multi::read(stream)?)),
            0x1B => Ok(Self::MultinameL(MultiL::read(stream)?)),
            0x1C => Ok(Self::MultinameLA(MultiL::read(stream)?)),
            0x1D => Ok(Self::Typename(Typename::read(stream)?)),
            kind => Err(RabcError::InvalidMultinameKind(kind)),
        }
    }

    pub fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u8(self.kind())?;
        match self {
            Self::QName(mn) => mn.write(stream),
            Self::QNameA(mn) => mn.write(stream),
            Self::RTQName(mn) => mn.write(stream),
            Self::RTQNameA(mn) => mn.write(stream),
            Self::RTQNameL(_) | Self::RTQNameLA(_) => Ok(()), // This kind has no associated data.
            Self::Multiname(mn) => mn.write(stream),
            Self::MultinameA(mn) => mn.write(stream),
            Self::MultinameL(mn) => mn.write(stream),
            Self::MultinameLA(mn) => mn.write(stream),
            Self::Typename(mn) => mn.write(stream),
        }
    }

    pub fn get_name_index(&self) -> Option<u32> {
        match self {
            Self::QName(mn) | Self::QNameA(mn) => Some(mn.name),
            Self::RTQName(mn) | Self::RTQNameA(mn) => Some(mn.name),
            Self::RTQNameL(_) | Self::RTQNameLA(_) => Some(0),
            Self::Multiname(mn) | Self::MultinameA(mn) => Some(mn.name),
            Self::MultinameL(_) | Self::MultinameLA(_) | Self::Typename(_) => None,
        }
    }
}

impl Default for Multiname {
    fn default() -> Self {
        Self::QName(QName::default())
    }
}
