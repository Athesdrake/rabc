use super::{Class, Multiname, Namespace};
use crate::{
    error::{RabcError, Result},
    StreamReader, StreamWriter,
};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ConstantPool {
    pub integers: Vec<i32>,
    pub uintegers: Vec<u32>,
    pub doubles: Vec<f64>,
    pub strings: Vec<String>,
    pub namespaces: Vec<Namespace>,
    pub ns_sets: Vec<Vec<u32>>,
    pub multinames: Vec<Multiname>,
}

impl ConstantPool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        let mut capacity = stream.read_u30()?.saturating_sub(1) + 1;
        let mut integers = Vec::with_capacity(capacity as usize);
        integers.push(0);
        for _ in 1..capacity {
            integers.push(stream.read_i30()?);
        }

        capacity = stream.read_u30()?.saturating_sub(1) + 1;
        let mut uintegers = Vec::with_capacity(capacity as usize);
        uintegers.push(0);
        for _ in 1..capacity {
            uintegers.push(stream.read_u30()?);
        }

        capacity = stream.read_u30()?.saturating_sub(1) + 1;
        let mut doubles = Vec::with_capacity(capacity as usize);
        doubles.push(0.0);
        for _ in 1..capacity {
            doubles.push(stream.read_double()?);
        }

        capacity = stream.read_u30()?.saturating_sub(1) + 1;
        let mut strings = Vec::with_capacity(capacity as usize);
        strings.push(String::new());
        for _ in 1..capacity {
            strings.push(stream.read_string()?);
        }

        capacity = stream.read_u30()?.saturating_sub(1) + 1;
        let mut namespaces = Vec::with_capacity(capacity as usize);
        namespaces.push(Namespace::new());
        for _ in 1..capacity {
            namespaces.push(Namespace::read(stream)?);
        }

        capacity = stream.read_u30()?.saturating_sub(1) + 1;
        let mut ns_sets = Vec::with_capacity(capacity as usize);
        ns_sets.push(Vec::new());
        for _ in 1..capacity {
            let ns_count = stream.read_u30()?;
            let mut ns_set = Vec::with_capacity(ns_count as usize);
            for _ in 0..ns_count {
                ns_set.push(stream.read_u30()?);
            }
            ns_sets.push(ns_set);
        }

        capacity = stream.read_u30()?.saturating_sub(1) + 1;
        let mut multinames = Vec::with_capacity(capacity as usize);
        multinames.push(Multiname::new());
        for _ in 1..capacity {
            multinames.push(Multiname::read(stream)?);
        }

        Ok(Self {
            integers,
            uintegers,
            doubles,
            strings,
            namespaces,
            ns_sets,
            multinames,
        })
    }

    pub fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.array_size(self.integers.len()))?;
        for value in self.integers.iter().skip(1) {
            stream.write_i30(*value)?;
        }
        stream.write_u30(self.array_size(self.uintegers.len()))?;
        for value in self.uintegers.iter().skip(1) {
            stream.write_u30(*value)?;
        }
        stream.write_u30(self.array_size(self.doubles.len()))?;
        for value in self.doubles.iter().skip(1) {
            stream.write_double(*value)?;
        }
        stream.write_u30(self.array_size(self.strings.len()))?;
        for value in self.strings.iter().skip(1) {
            stream.write_string(value)?;
        }

        stream.write_u30(self.array_size(self.namespaces.len()))?;
        for ns in self.namespaces.iter().skip(1) {
            ns.write(stream)?;
        }
        stream.write_u30(self.array_size(self.ns_sets.len()))?;
        for ns_set in self.ns_sets.iter().skip(1) {
            stream.write_u30(ns_set.len() as u32)?;
            for ns in ns_set {
                stream.write_u30(*ns)?;
            }
        }
        stream.write_u30(self.array_size(self.multinames.len()))?;
        for mn in self.multinames.iter().skip(1) {
            mn.write(stream)?;
        }

        Ok(())
    }

    pub fn get_int(&self, index: u32) -> Result<&i32> {
        Self::get("integers", &self.integers, index)
    }
    pub fn get_uint(&self, index: u32) -> Result<&u32> {
        Self::get("uintegers", &self.uintegers, index)
    }
    pub fn get_double(&self, index: u32) -> Result<&f64> {
        Self::get("doubles", &self.doubles, index)
    }
    pub fn get_str(&self, index: u32) -> Result<&String> {
        Self::get("strings", &self.strings, index)
    }
    pub fn get_ns(&self, index: u32) -> Result<&Namespace> {
        Self::get("namespaces", &self.namespaces, index)
    }
    pub fn get_ns_set(&self, index: u32) -> Result<&Vec<u32>> {
        Self::get("ns_sets", &self.ns_sets, index)
    }
    pub fn get_mn(&self, index: u32) -> Result<&Multiname> {
        Self::get("multinames", &self.multinames, index)
    }

    pub fn get_int_mut(&mut self, index: u32) -> Result<&mut i32> {
        Self::get_mut("integers", &mut self.integers, index)
    }
    pub fn get_uint_mut(&mut self, index: u32) -> Result<&mut u32> {
        Self::get_mut("uintegers", &mut self.uintegers, index)
    }
    pub fn get_double_mut(&mut self, index: u32) -> Result<&mut f64> {
        Self::get_mut("doubles", &mut self.doubles, index)
    }
    pub fn get_str_mut(&mut self, index: u32) -> Result<&mut String> {
        Self::get_mut("strings", &mut self.strings, index)
    }
    pub fn get_ns_mut(&mut self, index: u32) -> Result<&mut Namespace> {
        Self::get_mut("namespaces", &mut self.namespaces, index)
    }
    pub fn get_ns_set_mut(&mut self, index: u32) -> Result<&mut Vec<u32>> {
        Self::get_mut("ns_sets", &mut self.ns_sets, index)
    }
    pub fn get_mn_mut(&mut self, index: u32) -> Result<&mut Multiname> {
        Self::get_mut("multinames", &mut self.multinames, index)
    }

    #[inline]
    fn get<'a, T>(name: &'static str, container: &'a [T], index: u32) -> Result<&'a T> {
        let size = container.len();
        container
            .get(index as usize)
            .ok_or(RabcError::IndexOutOfBounds(name, index as usize, size))
    }
    #[inline]
    fn get_mut<'a, T>(name: &'static str, container: &'a mut [T], index: u32) -> Result<&'a mut T> {
        let size = container.len();
        container
            .get_mut(index as usize)
            .ok_or(RabcError::IndexOutOfBounds(name, index as usize, size))
    }

    #[inline]
    fn array_size(&self, length: usize) -> u32 {
        if length > 1 {
            length as u32
        } else {
            0u32
        }
    }

    /// Get the name of QName as a ref from the multiname index
    pub fn qname(&self, index: u32) -> Option<String> {
        self.qname_from_mn(self.get_mn(index).ok()?)
    }
    /// Get the name of QName as a ref
    pub fn qname_from_mn(&self, mn: &Multiname) -> Option<String> {
        match mn.get_name_index()? {
            0 => None,
            index => Some(self.get_str(index).ok()?.to_string()),
        }
    }
    /// Get a multiname's name from its index
    pub fn str(&self, index: u32) -> Option<String> {
        self.str_from_mn(self.get_mn(index).ok()?)
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
                self.str_from_ns_set(self.get_ns_set(m.ns_set).ok()?)
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
            let ns = self.get_ns(*index).ok()?;
            name.push_str(&self.str_from_ns(ns)?);
        }
        Some(name)
    }
    /// Get a namespace's name
    pub fn str_from_ns(&self, ns: &Namespace) -> Option<String> {
        self.get_str(ns.name).ok().cloned()
    }
    /// Get a multiname's namespace's name
    pub fn ns(&self, mn: &Multiname) -> Option<String> {
        match mn {
            Multiname::QName(mn) | Multiname::QNameA(mn) => {
                if mn.ns == 0 {
                    Some(String::new())
                } else {
                    self.str_from_ns(self.get_ns(mn.ns).ok()?)
                }
            }
            Multiname::Multiname(mn) | Multiname::MultinameA(mn) => {
                self.str_from_ns_set(self.get_ns_set(mn.ns_set).ok()?)
            }
            Multiname::MultinameL(mn) | Multiname::MultinameLA(mn) => {
                self.str_from_ns_set(self.get_ns_set(mn.ns_set).ok()?)
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
        let mn = self.get_mn(klass.name).ok()?;
        let mut name = self.ns(mn)?;
        if !name.is_empty() {
            name += "::";
        }
        name += match mn {
            Multiname::QName(qname) | Multiname::QNameA(qname) => {
                if qname.name == 0 {
                    "*"
                } else {
                    self.get_str(qname.name).ok()?
                }
            }
            _ => unreachable!(),
        };
        Some(name)
    }
}

pub trait PushGetIndex<T> {
    fn pushi(&mut self, value: T) -> u32;
}

impl<T> PushGetIndex<T> for Vec<T> {
    #[inline]
    fn pushi(&mut self, value: T) -> u32 {
        let index = self.len() as u32;
        self.push(value);
        index
    }
}
