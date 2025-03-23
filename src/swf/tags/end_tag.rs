use super::ITag;
use crate::{error::Result, Movie, StreamReader, StreamWriter};

#[derive(Clone, Debug, PartialEq)]
pub struct EndTag {}

impl ITag for EndTag {
    fn read(_stream: &mut StreamReader) -> Result<Self> {
        Ok(Self {})
    }
    fn write(&self, _stream: &mut StreamWriter, _movie: &Movie) -> Result<()> {
        Ok(())
    }
}
