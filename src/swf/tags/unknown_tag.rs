use super::ITag;
use crate::{error::Result, Movie, StreamReader, StreamWriter};

#[derive(Debug, PartialEq)]
pub struct UnknownTag {
    pub id: u16,
    pub data: StreamReader,
}

impl UnknownTag {
    pub fn read_with_id(stream: &mut StreamReader, tag_id: u16) -> Result<Self> {
        Ok(Self {
            id: tag_id,
            data: stream.copy()?,
        })
    }
}

impl ITag for UnknownTag {
    fn read(stream: &mut StreamReader) -> Result<Self> {
        Ok(Self {
            id: 0,
            data: stream.copy()?,
        })
    }

    fn write(&self, stream: &mut StreamWriter, _movie: &Movie) -> Result<()> {
        Ok(self.data.write_to_stream(stream)?)
    }
}
