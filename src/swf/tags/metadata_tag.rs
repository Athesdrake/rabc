use super::ITag;
use crate::{error::Result, Movie, StreamReader, StreamWriter};

#[derive(Debug, PartialEq)]
pub struct MetadataTag {
    /// Metadata as xml
    pub metadata: String,
}

impl ITag for MetadataTag {
    fn read(stream: &mut StreamReader) -> Result<Self> {
        Ok(Self {
            metadata: stream.read_null_string()?,
        })
    }

    fn write(&self, stream: &mut StreamWriter, _movie: &Movie) -> Result<()> {
        stream.write_null_string(&self.metadata)
    }
}
