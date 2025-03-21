use super::ITag;
use crate::{error::Result, Movie, StreamReader, StreamWriter};

#[derive(Debug, PartialEq)]
pub struct UnknownTag {
    pub id: u16,
    pub data: Vec<u8>,
}

impl UnknownTag {
    pub fn read_with_id(data: &[u8], tag_id: u16) -> Result<Self> {
        Ok(Self {
            id: tag_id,
            data: data.to_vec(),
        })
    }
}

impl ITag for UnknownTag {
    fn read(_stream: &mut StreamReader) -> Result<Self> {
        unreachable!("Unknown tags should not be read from this trait.")
    }

    fn write(&self, stream: &mut StreamWriter, _movie: &Movie) -> Result<()> {
        stream.write_exact(&self.data)
    }
}
