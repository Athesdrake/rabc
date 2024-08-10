use super::info::*;
use crate::error::{RabcError, Result};
use crate::{StreamReader, StreamWriter};

#[derive(Debug, PartialEq, Default)]
pub struct AbcVersion {
    pub major: u16,
    pub minor: u16,
}

/// Represent an ABC file which contains compiled programs: constant data, instructions and various kinds of metdata
#[derive(Debug, PartialEq, Default)]
pub struct AbcFile {
    pub version: AbcVersion,

    /// abc constant pool, contains all constant values
    pub cpool: ConstantPool,

    pub methods: Vec<Method>,
    pub classes: Vec<Class>,
    pub scripts: Vec<Script>,
    pub metadatas: Vec<Metadata>,
}

impl AbcFile {
    pub fn new() -> Self {
        Self::default()
    }

    /// Read the abc content from a stream
    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        let version = AbcVersion::read(stream)?;
        let cpool = ConstantPool::read(stream)?;

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
            version,
            cpool,
            methods,
            classes,
            scripts,
            metadatas,
        })
    }

    /// Write the abc content to a stream
    pub fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        self.version.write(stream)?;
        self.cpool.write(stream)?;

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

    /// Get the name of QName as a ref from the multiname index
    pub fn qname(&self, index: u32) -> Option<String> {
        self.qname_from_mn(self.cpool.multinames.get(index as usize)?)
    }
    /// Get the name of QName as a ref
    pub fn qname_from_mn(&self, mn: &Multiname) -> Option<String> {
        let index = mn.get_name_index().unwrap_or(0);
        if index == 0 {
            None
        } else {
            Some(self.cpool.strings.get(index as usize)?.to_string())
        }
    }
    /// Get a multiname's name from its index
    pub fn str(&self, index: u32) -> Option<String> {
        self.str_from_mn(self.cpool.multinames.get(index as usize)?)
    }
    /// Get a multiname's name
    pub fn str_from_mn(&self, mn: &Multiname) -> Option<String> {
        match mn {
            Multiname::Multiname(_)
            | Multiname::MultinameA(_)
            | Multiname::QName(_)
            | Multiname::QNameA(_)
            | Multiname::RTQName(_)
            | Multiname::RTQNameA(_)
            | Multiname::RTQNameL(_)
            | Multiname::RTQNameLA(_) => self.qname_from_mn(mn),
            Multiname::MultinameL(m) | Multiname::MultinameLA(m) => {
                self.str_from_ns_set(self.cpool.ns_sets.get(m.ns_set as usize)?)
            }
            Multiname::Typename(t) => {
                let mut types = String::new();
                if !t.types.is_empty() {
                    types.push('<');
                    for type_ in &t.types {
                        types.push_str(&self.str(*type_)?);
                    }
                    types.push('>');
                }
                Some(self.str(t.qname)? + &types)
            }
        }
    }
    /// Get a namespace set's name
    pub fn str_from_ns_set(&self, ns_set: &Vec<u32>) -> Option<String> {
        let mut name = String::new();
        for index in ns_set {
            if !name.is_empty() {
                name.push_str("::");
            }
            let ns = self.cpool.namespaces.get(index.to_be() as usize)?;
            name.push_str(&self.str_from_ns(ns)?);
        }
        Some(name)
    }
    /// Get a namespace's name
    pub fn str_from_ns(&self, ns: &Namespace) -> Option<String> {
        self.cpool.strings.get(ns.name as usize).cloned()
    }
    /// Get a multiname's namespace's name
    pub fn ns(&self, mn: &Multiname) -> Option<String> {
        match mn {
            Multiname::QName(mn) | Multiname::QNameA(mn) => {
                if mn.ns == 0 {
                    Some(String::new())
                } else {
                    self.str_from_ns(self.cpool.namespaces.get(mn.ns as usize)?)
                }
            }
            Multiname::Multiname(mn) | Multiname::MultinameA(mn) => {
                self.str_from_ns_set(self.cpool.ns_sets.get(mn.ns_set as usize)?)
            }
            Multiname::MultinameL(mn) | Multiname::MultinameLA(mn) => {
                self.str_from_ns_set(self.cpool.ns_sets.get(mn.ns_set as usize)?)
            }
            Multiname::RTQName(_)
            | Multiname::RTQNameA(_)
            | Multiname::RTQNameL(_)
            | Multiname::RTQNameLA(_)
            | Multiname::Typename(_) => Some(String::new()),
        }
    }
    /// Get the fully qualified name of a class: package::ClassName
    pub fn fqn(&self, klass: &Class) -> Option<String> {
        let mn = self.cpool.multinames.get(klass.name as usize)?;
        let mut name = self.ns(mn)?;
        if !name.is_empty() {
            name += "::";
        }
        name += match mn {
            Multiname::QName(qname) | Multiname::QNameA(qname) => {
                if qname.name == 0 {
                    "*"
                } else {
                    self.cpool.strings.get(qname.name as usize)?
                }
            }
            _ => unreachable!(),
        };
        Some(name)
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
