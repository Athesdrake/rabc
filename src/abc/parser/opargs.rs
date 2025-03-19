use crate::{error::Result, StreamReader, StreamWriter};

#[derive(Clone, Debug)]
pub struct ArgsCountArg {
    pub arg_count: u32,
}
#[derive(Clone, Debug)]
pub struct MultinameArg {
    pub mn: u32,
}
#[derive(Clone, Debug)]
pub struct CallPropertyArg {
    pub property: u32,
    pub arg_count: u32,
}
#[derive(Clone, Debug)]
pub struct CallMethodArg {
    pub method: u32,
    pub arg_count: u32,
}
#[derive(Clone, Debug)]
pub struct CallMethodDispArg {
    pub disp_id: u32,
    pub arg_count: u32,
}
#[derive(Clone, Debug)]
pub struct PropertyArg {
    pub property: u32,
}
#[derive(Clone, Debug)]
pub struct LineArg {
    pub line: u32,
}
#[derive(Clone, Debug)]
pub struct RegisterArg {
    pub register: u32,
}
#[derive(Clone, Debug)]
pub struct SlotArg {
    pub slot: u32,
}
#[derive(Clone, Debug)]
pub struct ScopeArg {
    pub scope: u32,
}
#[derive(Clone, Debug)]
pub struct TargetArg {
    pub target: u32,
}
#[derive(Clone, Debug)]
pub struct CoerceArg {
    pub index: u32,
}
#[derive(Clone, Debug)]
pub struct DebugArg {
    pub debug_type: u8,
    pub reg_name: u32,
    pub register: u8,
    pub extra: u32,
}
#[derive(Clone, Debug)]
pub struct DebugFileArg {
    pub filename: u32,
}
#[derive(Clone, Debug)]
pub struct DxnsArg {
    pub uri: u32,
}
#[derive(Clone, Debug)]
pub struct GetDescendantsArg {
    pub operand: u32,
}
#[derive(Clone, Debug)]
pub struct HasNext2Arg {
    pub object_register: u32,
    pub index_register: u32,
}
#[derive(Clone, Debug)]
pub struct LookupSwitchArg {
    pub default_target: u32,
    pub targets: Box<[u32]>,
}
#[derive(Clone, Debug)]
pub struct NewCatchArg {
    pub exception: u32,
}
#[derive(Clone, Debug)]
pub struct NewClassArg {
    pub class: u32,
}
#[derive(Clone, Debug)]
pub struct NewFunctionArg {
    pub method: u32,
}
#[derive(Clone, Debug)]
pub struct NewObjectArg {
    pub property_count: u32,
}
#[derive(Clone, Debug)]
pub struct PushByteArg {
    pub value: u8,
}
#[derive(Clone, Debug)]
pub struct PushDoubleArg {
    pub value: u32,
}
#[derive(Clone, Debug)]
pub struct PushIntArg {
    pub value: u32,
}
#[derive(Clone, Debug)]
pub struct NamespaceArg {
    pub ns: u32,
}
#[derive(Clone, Debug)]
pub struct PushShortArg {
    pub value: i16,
}
#[derive(Clone, Debug)]
pub struct PushStringArg {
    pub value: u32,
}
#[derive(Clone, Debug)]
pub struct PushUintArg {
    pub value: u32,
}

pub trait SerializeTrait {
    fn parse(stream: &mut StreamReader) -> Result<Self>
    where
        Self: Sized;
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()>;
}
impl SerializeTrait for MultinameArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let mn = stream.read_u30()?;
        Ok(Self { mn })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.mn)
    }
}
impl SerializeTrait for RegisterArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let register = stream.read_u30()?;
        Ok(Self { register })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.register)
    }
}
impl SerializeTrait for TargetArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let target = (stream.read_i24()? + stream.pos() as i32) as u32;
        Ok(Self { target })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_i24((self.target as i32) - (stream.len() as i32 + 3))
    }
}
impl SerializeTrait for CallPropertyArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        Ok(Self {
            property: stream.read_u30()?,
            arg_count: stream.read_u30()?,
        })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.property)?;
        stream.write_u30(self.arg_count)
    }
}
impl SerializeTrait for ArgsCountArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let arg_count = stream.read_u30()?;
        Ok(Self { arg_count })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.arg_count)
    }
}
impl SerializeTrait for PropertyArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let property = stream.read_u30()?;
        Ok(Self { property })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.property)
    }
}
impl SerializeTrait for ScopeArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let scope = stream.read_u30()?;
        Ok(Self { scope })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.scope)
    }
}
impl SerializeTrait for SlotArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let slot = stream.read_u30()?;
        Ok(Self { slot })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.slot)
    }
}
impl SerializeTrait for CallMethodArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let method = stream.read_u30()?;
        let arg_count = stream.read_u30()?;
        Ok(Self { method, arg_count })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.method)?;
        stream.write_u30(self.arg_count)
    }
}
impl SerializeTrait for LineArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let line = stream.read_u30()?;
        Ok(Self { line })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.line)
    }
}
impl SerializeTrait for CallMethodDispArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let disp_id = stream.read_u30()?;
        let arg_count = stream.read_u30()?;
        Ok(Self { disp_id, arg_count })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.disp_id)?;
        stream.write_u30(self.arg_count)
    }
}
impl SerializeTrait for CoerceArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let index = stream.read_u30()?;
        Ok(Self { index })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.index)
    }
}
impl SerializeTrait for DebugArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        Ok(Self {
            debug_type: stream.read_u8()?,
            reg_name: stream.read_u30()?,
            register: stream.read_u8()?,
            extra: stream.read_u30()?,
        })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u8(self.debug_type)?;
        stream.write_u30(self.reg_name)?;
        stream.write_u8(self.register)?;
        stream.write_u30(self.extra)
    }
}
impl SerializeTrait for DebugFileArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let filename = stream.read_u30()?;
        Ok(Self { filename })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.filename)
    }
}
impl SerializeTrait for DxnsArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let uri = stream.read_u30()?;
        Ok(Self { uri })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.uri)
    }
}
impl SerializeTrait for GetDescendantsArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let operand = stream.read_u30()?;
        Ok(Self { operand })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.operand)
    }
}
impl SerializeTrait for HasNext2Arg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        Ok(Self {
            object_register: stream.read_u30()?,
            index_register: stream.read_u30()?,
        })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.object_register)?;
        stream.write_u30(self.index_register)
    }
}
impl SerializeTrait for LookupSwitchArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        // The base address for the lookupswitch's targets is always the instruction's address unlike all other jump
        // instructions.
        let addr = stream.pos() as i32 - 1;
        let default_target = (addr + stream.read_i24()?) as u32;
        let cases = stream.read_u30()?;
        let mut targets = Vec::with_capacity(cases as usize);
        for _ in 0..=cases {
            targets.push((addr + stream.read_i24()?) as u32);
        }
        Ok(Self {
            default_target,
            targets: targets.into(),
        })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        let addr = (stream.len() - 1) as i32;
        stream.write_u30((self.default_target as i32 - addr) as u32)?;
        stream.write_u30(self.targets.len() as u32)?;
        for target in &self.targets {
            stream.write_i24(*target as i32 - addr)?;
        }
        Ok(())
    }
}
impl SerializeTrait for NewCatchArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let exception = stream.read_u30()?;
        Ok(Self { exception })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.exception)
    }
}
impl SerializeTrait for NewClassArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let class = stream.read_u30()?;
        Ok(Self { class })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.class)
    }
}
impl SerializeTrait for NewFunctionArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let method = stream.read_u30()?;
        Ok(Self { method })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.method)
    }
}
impl SerializeTrait for NewObjectArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let property_count = stream.read_u30()?;
        Ok(Self { property_count })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.property_count)
    }
}
impl SerializeTrait for PushByteArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let value = stream.read_u8()?;
        Ok(Self { value })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u8(self.value)
    }
}
impl SerializeTrait for PushDoubleArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let value = stream.read_u30()?;
        Ok(Self { value })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.value)
    }
}
impl SerializeTrait for PushIntArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let value = stream.read_u30()?;
        Ok(Self { value })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.value)
    }
}
impl SerializeTrait for NamespaceArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let ns = stream.read_u30()?;
        Ok(Self { ns })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.ns)
    }
}
impl SerializeTrait for PushShortArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let value = stream.read_u30()? as i16;
        Ok(Self { value })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.value as u32)
    }
}
impl SerializeTrait for PushStringArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let value = stream.read_u30()?;
        Ok(Self { value })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.value)
    }
}
impl SerializeTrait for PushUintArg {
    fn parse(stream: &mut StreamReader) -> Result<Self> {
        let value = stream.read_u30()?;
        Ok(Self { value })
    }
    fn serialize(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.value)
    }
}
