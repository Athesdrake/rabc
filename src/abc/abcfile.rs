use super::info::*;
use crate::error::{RabcError, Result};
use crate::{StreamReader, StreamWriter};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AbcVersion {
    pub major: u16,
    pub minor: u16,
}

/// Represent an ABC file which contains compiled programs: constant data, instructions and various kinds of metdata
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AbcFile {
    /// AVM2 version
    pub version: AbcVersion,
    /// abc constant pool, contains all constant values
    pub cpool: ConstantPool,
    /// abc data, contains all scripts
    pub abc: Abc,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Abc {
    pub classes: Vec<Class>,
    pub metadatas: Vec<Metadata>,
    pub methods: Vec<Method>,
    pub scripts: Vec<Script>,
}

impl AbcFile {
    pub fn new() -> Self {
        Self::default()
    }

    /// Read the abc content from a stream
    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        let version = AbcVersion::read(stream)?;
        let cpool = ConstantPool::read(stream)?;
        let abc = Abc::read(stream)?;

        Ok(Self {
            version,
            cpool,
            abc,
        })
    }

    /// Write the abc content to a stream
    pub fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        self.version.write(stream)?;
        self.cpool.write(stream)?;
        self.abc.write(stream)?;
        Ok(())
    }
}

impl Abc {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        let mut count = stream.read_u30()?;
        let mut methods = Vec::with_capacity(count as usize);
        for _ in 0..count {
            methods.push(Method::read(stream)?);
        }

        count = stream.read_u30()?;
        let mut metadatas = Vec::with_capacity(count as usize);
        for _ in 0..count {
            metadatas.push(Metadata::read(stream)?);
        }

        count = stream.read_u30()?;
        let mut classes = Vec::with_capacity(count as usize);
        for _ in 0..count {
            classes.push(Class::read_instance(stream)?);
        }
        for class in &mut classes {
            class.read(stream)?;
        }

        count = stream.read_u30()?;
        let mut scripts = Vec::with_capacity(count as usize);
        for _ in 0..count {
            scripts.push(Script::read(stream)?);
        }

        count = stream.read_u30()?;
        for _ in 0..count {
            let index = stream.read_u30()?;
            let method = methods
                .get_mut(index as usize)
                .ok_or(RabcError::MethodOutOfBound(index))?;
            method.read_body(stream)?;
        }

        Ok(Self {
            classes,
            metadatas,
            methods,
            scripts,
        })
    }

    pub fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        let mut bodies = Vec::with_capacity(self.methods.len());
        stream.write_u30(self.methods.len() as u32)?;
        for (i, method) in self.methods.iter().enumerate() {
            method.write(stream)?;
            if method.has_body() {
                bodies.push(i);
            }
        }

        stream.write_u30(self.metadatas.len() as u32)?;
        for metadata in &self.metadatas {
            metadata.write(stream)?;
        }

        stream.write_u30(self.classes.len() as u32)?;
        for class in &self.classes {
            class.write_instance(stream)?;
        }
        for class in &self.classes {
            class.write(stream)?;
        }

        stream.write_u30(self.scripts.len() as u32)?;
        for script in &self.scripts {
            script.write(stream)?;
        }

        stream.write_u30(bodies.len() as u32)?;
        for i in bodies {
            stream.write_u30(i as u32)?;
            self.methods[i].write_body(stream)?;
        }
        Ok(())
    }

    #[inline]
    pub fn get_method(&self, index: u32) -> Result<&Method> {
        self.methods
            .get(index as usize)
            .ok_or(RabcError::IndexOutOfBounds(
                "methods",
                index as usize,
                self.methods.len(),
            ))
    }
    #[inline]
    pub fn get_class(&self, index: u32) -> Result<&Class> {
        self.classes
            .get(index as usize)
            .ok_or(RabcError::IndexOutOfBounds(
                "classes",
                index as usize,
                self.classes.len(),
            ))
    }
    #[inline]
    pub fn get_script(&self, index: u32) -> Result<&Script> {
        self.scripts
            .get(index as usize)
            .ok_or(RabcError::IndexOutOfBounds(
                "scripts",
                index as usize,
                self.scripts.len(),
            ))
    }
    #[inline]
    pub fn get_metadata(&self, index: u32) -> Result<&Metadata> {
        self.metadatas
            .get(index as usize)
            .ok_or(RabcError::IndexOutOfBounds(
                "metadatas",
                index as usize,
                self.metadatas.len(),
            ))
    }
}

impl AbcVersion {
    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        Ok(Self {
            minor: stream.read_u16()?,
            major: stream.read_u16()?,
        })
    }

    pub fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u16(self.minor)?;
        stream.write_u16(self.major)?;
        Ok(())
    }
}
