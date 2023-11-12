use super::Trait;
use crate::{error::Result, StreamReader, StreamWriter};
use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ClassFlag : u8 {
        const SEALED = 0x01;
        const FINAL = 0x02;
        const INTERFACE = 0x04;
        const PROTECTED_NAMESPACE = 0x08;
    }
}

#[derive(PartialEq, Debug)]
pub struct Class {
    pub name: u32,
    pub super_name: u32,
    pub flags: ClassFlag,
    pub protected_ns: u32,
    pub iinit: u32,
    pub cinit: u32,
    pub interfaces: Vec<u32>,
    pub itraits: Vec<Trait>,
    pub ctraits: Vec<Trait>,
}

impl Class {
    #[inline]
    pub fn read_instance(stream: &mut StreamReader) -> Result<Self> {
        let name = stream.read_u30()?;
        let super_name = stream.read_u30()?;
        let flags = ClassFlag::from_bits_retain(stream.read_u8()?);
        let protected_ns = if flags.contains(ClassFlag::PROTECTED_NAMESPACE) {
            stream.read_u30()?
        } else {
            0
        };

        let mut count = stream.read_u30()?;
        let mut interfaces = Vec::with_capacity(count as usize);
        for _ in 0..count {
            interfaces.push(stream.read_u30()?);
        }
        let iinit = stream.read_u30()?;

        count = stream.read_u30()?;
        let mut itraits = Vec::with_capacity(count as usize);
        for _ in 0..count {
            itraits.push(Trait::read(stream)?);
        }

        Ok(Self {
            name,
            super_name,
            flags,
            protected_ns,
            iinit,
            interfaces,
            itraits,
            cinit: 0,
            ctraits: Vec::new(),
        })
    }

    pub fn read(&mut self, stream: &mut StreamReader) -> Result<()> {
        self.cinit = stream.read_u30()?;
        let count = stream.read_u30()?;

        self.ctraits.reserve_exact(count as usize);
        for _ in 0..count {
            self.ctraits.push(Trait::read(stream)?);
        }
        Ok(())
    }

    pub fn write_instance(&self, stream: &mut StreamWriter) -> Result<()> {
        let is_protected = self.protected_ns != 0;
        // self.flags.set(ClassFlag::PROTECTED_NAMESPACE, is_protected);

        stream.write_u30(self.name)?;
        stream.write_u30(self.super_name)?;
        stream.write_u8(self.flags.bits() as u8)?;

        if is_protected {
            stream.write_u30(self.protected_ns)?;
        }

        stream.write_u30(self.interfaces.len() as u32)?;
        for interface in &self.interfaces {
            stream.write_u30(*interface)?;
        }

        stream.write_u30(self.iinit)?;
        stream.write_u30(self.itraits.len() as u32)?;
        for t in &self.itraits {
            t.write(stream)?;
        }
        Ok(())
    }

    pub fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.cinit)?;
        stream.write_u30(self.ctraits.len() as u32)?;
        for t in &self.ctraits {
            t.write(stream)?;
        }
        Ok(())
    }

    /// The class is sealed : properties can not be dynamically added to instances of the class.
    pub fn is_sealed(&self) -> bool {
        self.flags.contains(ClassFlag::SEALED)
    }
    /// The class is final : it cannot be a base class for any other class.
    pub fn is_final(&self) -> bool {
        self.flags.contains(ClassFlag::FINAL)
    }
    /// The class is an interface.
    pub fn is_interface(&self) -> bool {
        self.flags.contains(ClassFlag::INTERFACE)
    }
    /// The class uses its protected namespace and the protectedNs field is present in the interface_info structure.
    pub fn is_protected(&self) -> bool {
        self.flags.contains(ClassFlag::PROTECTED_NAMESPACE)
    }

    pub fn set_sealed(&mut self, enable: bool) {
        self.flags.set(ClassFlag::SEALED, enable);
    }
    pub fn set_final(&mut self, enable: bool) {
        self.flags.set(ClassFlag::FINAL, enable);
    }
    pub fn set_interface(&mut self, enable: bool) {
        self.flags.set(ClassFlag::INTERFACE, enable);
    }
    pub fn set_protected(&mut self, enable: bool) {
        self.flags.set(ClassFlag::PROTECTED_NAMESPACE, enable);
    }
}
