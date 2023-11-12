use super::ITag;
use crate::{error::Result, swf::datatypes::Rgb, Movie, StreamReader, StreamWriter};

#[derive(Debug, PartialEq)]
pub struct SetBackgroundColorTag {
    pub color: Rgb,
}

impl ITag for SetBackgroundColorTag {
    fn read(stream: &mut StreamReader) -> Result<Self> {
        Ok(Self {
            color: Rgb::read(stream)?,
        })
    }

    fn write(&self, stream: &mut StreamWriter, _movie: &Movie) -> Result<()> {
        self.color.write(stream)?;
        Ok(())
    }
}
