use crate::{
    error::{RabcError, Result},
    StreamReader, StreamWriter,
};
use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct TraitAttr : u8 {
        const FINAL = 0x01;
        const OVERRIDE = 0x02;
        const METADATA = 0x04;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Trait {
    Slot(SlotTrait),
    Method(IndexTrait),
    Getter(IndexTrait),
    Setter(IndexTrait),
    Class(IndexTrait),
    Function(IndexTrait),
    Const(SlotTrait),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SlotTrait {
    pub name: u32,
    pub attr: TraitAttr,
    pub slot_id: u32,
    pub slot_type: u32,
    pub index: u32,
    pub kind: u8,
    pub metadatas: Vec<u32>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IndexTrait {
    pub name: u32,
    pub attr: TraitAttr,
    pub slot_id: u32,
    pub index: u32,
    pub metadatas: Vec<u32>,
}

impl Trait {
    pub fn kind(&self) -> u8 {
        self.into()
    }
    pub fn name(&self) -> u32 {
        match self {
            Trait::Slot(t) | Trait::Const(t) => t.name,
            Trait::Method(t)
            | Trait::Getter(t)
            | Trait::Setter(t)
            | Trait::Class(t)
            | Trait::Function(t) => t.name,
        }
    }
    pub fn metadatas(&self) -> &Vec<u32> {
        let t: &dyn ITrait = match self {
            Trait::Slot(t) | Trait::Const(t) => t,
            Trait::Method(t)
            | Trait::Getter(t)
            | Trait::Setter(t)
            | Trait::Class(t)
            | Trait::Function(t) => t,
        };
        t.metadatas()
    }
    pub fn metadatas_mut(&mut self) -> &mut Vec<u32> {
        let t: &mut dyn ITrait = match self {
            Trait::Slot(t) | Trait::Const(t) => t,
            Trait::Method(t)
            | Trait::Getter(t)
            | Trait::Setter(t)
            | Trait::Class(t)
            | Trait::Function(t) => t,
        };
        t.metadatas_mut()
    }

    #[inline]
    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        let name = stream.read_u30()?;
        let kkind = stream.read_u8()?;
        let attr = TraitAttr::from_bits_retain(kkind >> 4);
        match kkind & 0x0f {
            0 => Ok(Self::Slot(SlotTrait::read(stream, name, attr)?)),
            1 => Ok(Self::Method(IndexTrait::read(stream, name, attr)?)),
            2 => Ok(Self::Getter(IndexTrait::read(stream, name, attr)?)),
            3 => Ok(Self::Setter(IndexTrait::read(stream, name, attr)?)),
            4 => Ok(Self::Class(IndexTrait::read(stream, name, attr)?)),
            5 => Ok(Self::Function(IndexTrait::read(stream, name, attr)?)),
            6 => Ok(Self::Const(SlotTrait::read(stream, name, attr)?)),
            k => Err(RabcError::InvalidTraitKind(k)),
        }
    }

    #[inline]
    pub fn read_metadata(stream: &mut StreamReader, attr: &TraitAttr) -> Result<Vec<u32>> {
        if !attr.contains(TraitAttr::METADATA) {
            return Ok(Vec::new());
        }
        let count = stream.read_u30()?;
        let mut metadatas = Vec::with_capacity(count as usize);
        for _ in 0..count {
            metadatas.push(stream.read_u30()?);
        }
        Ok(metadatas)
    }

    #[inline]
    pub fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        let trait_: &dyn ITrait = match self {
            Trait::Slot(t) | Trait::Const(t) => t,
            Trait::Method(t)
            | Trait::Getter(t)
            | Trait::Setter(t)
            | Trait::Class(t)
            | Trait::Function(t) => t,
        };

        let metadatas = trait_.metadatas();
        let mut attr = trait_.attr();
        // Make attributes consistent with the data
        attr.set(TraitAttr::METADATA, !metadatas.is_empty());

        // Write the trait's header
        stream.write_u30(trait_.name())?;
        stream.write_u8(self.kind() | (attr.bits() << 4))?;

        // Write the trait's specific data
        trait_.write(stream)?;

        // Write metadatas if any
        if !metadatas.is_empty() {
            stream.write_u30(metadatas.len() as u32)?;
            for metadata in metadatas {
                stream.write_u30(*metadata)?;
            }
        }
        Ok(())
    }
}

impl From<&Trait> for u8 {
    fn from(value: &Trait) -> Self {
        match value {
            Trait::Slot(_) => 0,
            Trait::Method(_) => 1,
            Trait::Getter(_) => 2,
            Trait::Setter(_) => 3,
            Trait::Class(_) => 4,
            Trait::Function(_) => 5,
            Trait::Const(_) => 6,
        }
    }
}

pub trait ITrait {
    fn read(stream: &mut StreamReader, name: u32, attr: TraitAttr) -> Result<Self>
    where
        Self: Sized;

    fn write(&self, stream: &mut StreamWriter) -> Result<()>;
    fn attr(&self) -> TraitAttr;
    fn name(&self) -> u32;
    fn metadatas(&self) -> &Vec<u32>;
    fn metadatas_mut(&mut self) -> &mut Vec<u32>;
}

impl ITrait for SlotTrait {
    #[inline]
    fn read(stream: &mut StreamReader, name: u32, attr: TraitAttr) -> Result<Self> {
        let slot_id = stream.read_u30()?;
        let slot_type = stream.read_u30()?;
        let index = stream.read_u30()?;
        let kind = if index == 0 { 0 } else { stream.read_u8()? };
        let metadatas = Trait::read_metadata(stream, &attr)?;

        Ok(Self {
            name,
            attr,
            slot_id,
            slot_type,
            index,
            kind,
            metadatas,
        })
    }

    fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.slot_id)?;
        stream.write_u30(self.slot_type)?;
        stream.write_u30(self.index)?;
        if self.index != 0 {
            stream.write_u8(self.kind)?;
        }
        Ok(())
    }
    fn attr(&self) -> TraitAttr {
        self.attr
    }
    fn name(&self) -> u32 {
        self.name
    }
    fn metadatas(&self) -> &Vec<u32> {
        &self.metadatas
    }
    fn metadatas_mut(&mut self) -> &mut Vec<u32> {
        &mut self.metadatas
    }
}

impl ITrait for IndexTrait {
    #[inline]
    fn read(stream: &mut StreamReader, name: u32, attr: TraitAttr) -> Result<Self> {
        let slot_id = stream.read_u30()?;
        let index = stream.read_u30()?;
        let metadatas = Trait::read_metadata(stream, &attr)?;

        Ok(Self {
            name,
            attr,
            slot_id,
            index,
            metadatas,
        })
    }

    fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.slot_id)?;
        stream.write_u30(self.index)?;
        Ok(())
    }
    fn attr(&self) -> TraitAttr {
        self.attr
    }
    fn name(&self) -> u32 {
        self.name
    }
    fn metadatas(&self) -> &Vec<u32> {
        &self.metadatas
    }
    fn metadatas_mut(&mut self) -> &mut Vec<u32> {
        &mut self.metadatas
    }
}
