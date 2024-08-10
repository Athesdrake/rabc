use super::{Multiname, Namespace};
use crate::{
    error::{RabcError, Result},
    StreamReader, StreamWriter,
};

#[derive(Debug, PartialEq, Default)]
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
        self.get("integers", &self.integers, index)
    }
    pub fn get_uint(&self, index: u32) -> Result<&u32> {
        self.get("uintegers", &self.uintegers, index)
    }
    pub fn get_double(&self, index: u32) -> Result<&f64> {
        self.get("doubles", &self.doubles, index)
    }
    pub fn get_str(&self, index: u32) -> Result<&String> {
        self.get("strings", &self.strings, index)
    }
    pub fn get_ns(&self, index: u32) -> Result<&Namespace> {
        self.get("namespaces", &self.namespaces, index)
    }
    pub fn get_ns_set(&self, index: u32) -> Result<&Vec<u32>> {
        self.get("ns_sets", &self.ns_sets, index)
    }
    pub fn get_mn(&self, index: u32) -> Result<&Multiname> {
        self.get("multinames", &self.multinames, index)
    }

    #[inline]
    fn get<'a, T>(&'a self, name: &'static str, container: &'a [T], index: u32) -> Result<&T> {
        container
            .get(index as usize)
            .ok_or_else(|| RabcError::IndexOutOfBounds(name, index as usize, container.len()))
    }

    #[inline]
    fn array_size(&self, length: usize) -> u32 {
        if length > 1 {
            length as u32
        } else {
            0u32
        }
    }
}
