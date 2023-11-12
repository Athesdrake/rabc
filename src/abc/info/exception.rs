use crate::{error::Result, StreamReader, StreamWriter};

#[derive(PartialEq, Debug)]
pub struct Exception {
    pub from: u32,
    pub to: u32,
    pub target: u32,
    pub type_: u32,
    pub var_name: u32,
}

impl Exception {
    pub fn new() -> Self {
        Self {
            from: 0,
            to: 0,
            target: 0,
            type_: 0,
            var_name: 0,
        }
    }

    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        Ok(Self {
            from: stream.read_u30()?,
            to: stream.read_u30()?,
            target: stream.read_u30()?,
            type_: stream.read_u30()?,
            var_name: stream.read_u30()?,
        })
    }

    pub(crate) fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.from)?;
        stream.write_u30(self.to)?;
        stream.write_u30(self.target)?;
        stream.write_u30(self.type_)?;
        stream.write_u30(self.var_name)?;
        Ok(())
    }
}
