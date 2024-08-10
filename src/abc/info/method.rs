use super::{Exception, Trait};
use crate::{error::Result, StreamReader, StreamWriter};
use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct MethodFlag : u8 {
        const NEED_ARGUMENTS = 0x01;
        const NEED_ACTIVATION = 0x02;
        const NEED_REST = 0x04;
        const HAS_OPTIONAL = 0x08;
        const SET_DXNS = 0x40;
        const HAS_PARAM_NAMES = 0x80;
    }
}

impl Default for MethodFlag {
    fn default() -> Self {
        MethodFlag::empty()
    }
}

#[derive(PartialEq, Debug)]
pub struct Option {
    pub value: u32,
    pub kind: u8,
}

#[derive(PartialEq, Debug, Default)]
pub struct Method {
    pub return_type: u32,
    pub name: u32,
    pub flags: MethodFlag,
    pub optional: Vec<Option>,
    pub param_names: Vec<String>,
    pub params: Vec<u32>,

    pub max_stack: u32,
    pub local_count: u32,
    pub init_scope_depth: u32,
    pub max_scope_depth: u32,
    pub code: Vec<u8>,
    pub exceptions: Vec<Exception>,
    pub traits: Vec<Trait>,
}

impl Method {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        let mut count = stream.read_u30()?;
        let mut params = Vec::with_capacity(count as usize);
        let return_type = stream.read_u30()?;

        for _ in 0..count {
            params.push(stream.read_u30()?);
        }

        let name = stream.read_u30()?;
        let flags = MethodFlag::from_bits_retain(stream.read_u8()?);
        let mut optional = Vec::new();
        let mut param_names = Vec::new();

        if flags.contains(MethodFlag::HAS_OPTIONAL) {
            count = stream.read_u30()?;
            optional.reserve_exact(count as usize);

            for _ in 0..count {
                optional.push(Option {
                    value: stream.read_u30()?,
                    kind: stream.read_u8()?,
                });
            }
        }
        if flags.contains(MethodFlag::HAS_PARAM_NAMES) {
            count = stream.read_u30()?;
            param_names.reserve_exact(count as usize);

            for _ in 0..count {
                param_names.push(stream.read_string()?);
            }
        }

        Ok(Self {
            return_type,
            name,
            flags,
            optional,
            param_names,
            params,
            max_stack: 0,
            local_count: 0,
            init_scope_depth: 0,
            max_scope_depth: 0,
            code: Vec::new(),
            exceptions: Vec::new(),
            traits: Vec::new(),
        })
    }

    pub fn read_body(&mut self, stream: &mut StreamReader) -> Result<()> {
        self.max_stack = stream.read_u30()?;
        self.local_count = stream.read_u30()?;
        self.init_scope_depth = stream.read_u30()?;
        self.max_scope_depth = stream.read_u30()?;
        let mut count = stream.read_u30()?;
        self.code.resize_with(count as usize, Default::default);
        stream.read_exact(&mut self.code)?;

        count = stream.read_u30()?;
        self.exceptions.reserve_exact(count as usize);
        for _ in 0..count {
            self.exceptions.push(Exception::read(stream)?);
        }

        count = stream.read_u30()?;
        self.traits.reserve_exact(count as usize);
        for _ in 0..count {
            self.traits.push(Trait::read(stream)?);
        }
        Ok(())
    }

    pub fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.params.len() as u32)?;
        stream.write_u30(self.return_type)?;

        for param in &self.params {
            stream.write_u30(*param)?;
        }
        stream.write_u30(self.name)?;

        let has_optional = !self.optional.is_empty();
        let has_param_names = !self.param_names.is_empty();
        // self.flags.set(MethodFlag::HAS_OPTIONAL, has_optional);
        // self.flags.set(MethodFlag::HAS_PARAM_NAMES, has_param_names);
        stream.write_u8(self.flags.bits())?;

        if has_optional {
            stream.write_u30(self.optional.len() as u32)?;
            for option in &self.optional {
                stream.write_u30(option.value)?;
                stream.write_u8(option.kind)?;
            }
        }
        if has_param_names {
            stream.write_u30(self.param_names.len() as u32)?;
            for param in &self.param_names {
                stream.write_string(param)?;
            }
        }
        Ok(())
    }

    pub fn write_body(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.max_stack)?;
        stream.write_u30(self.local_count)?;
        stream.write_u30(self.init_scope_depth)?;
        stream.write_u30(self.max_scope_depth)?;
        stream.write_u30(self.code.len() as u32)?;
        stream.write_exact(&self.code)?;

        stream.write_u30(self.exceptions.len() as u32)?;
        for e in &self.exceptions {
            e.write(stream)?;
        }

        stream.write_u30(self.traits.len() as u32)?;
        for t in &self.traits {
            t.write(stream)?;
        }
        Ok(())
    }

    pub fn has_body(&self) -> bool {
        !self.code.is_empty()
    }
    pub fn has_optional(&self) -> bool {
        self.flags.contains(MethodFlag::HAS_OPTIONAL)
    }
    pub fn has_param_names(&self) -> bool {
        self.flags.contains(MethodFlag::HAS_PARAM_NAMES)
    }
    pub fn need_rest(&self) -> bool {
        self.flags.contains(MethodFlag::NEED_REST)
    }
}
