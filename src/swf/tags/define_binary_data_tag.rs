use super::ITag;
use crate::{error::Result, Movie, StreamReader, StreamWriter};

#[derive(Debug, PartialEq)]
pub struct DefineBinaryDataTag {
    pub char_id: u16,
    pub data: Vec<u8>,
}

impl ITag for DefineBinaryDataTag {
    fn read(stream: &mut StreamReader) -> Result<Self> {
        let char_id = stream.read_u16()?;
        let _reserved = stream.read_u32()?;
        let length = stream.remaining() as usize;
        let mut data = vec![0u8; length];
        stream.read_exact(data.as_mut())?;

        Ok(Self { char_id, data })
    }

    fn write(&self, stream: &mut StreamWriter, _movie: &Movie) -> Result<()> {
        stream.write_u16(self.char_id)?;
        stream.write_u32(0)?;
        stream.write_exact(&self.data)?;
        Ok(())
    }
}
