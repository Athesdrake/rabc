use super::ITag;
use crate::{abc::AbcFile, error::Result, Movie, StreamReader, StreamWriter};

#[derive(Debug, PartialEq)]
pub struct DoABCTag {
    pub lazy: bool,
    pub name: String,
    pub abcfile: AbcFile,
}

impl ITag for DoABCTag {
    fn read(stream: &mut StreamReader) -> Result<Self> {
        let lazy = stream.read_u32()? & 1 == 1;
        let name = stream.read_null_string()?;
        let abcfile = AbcFile::read(stream)?;

        Ok(Self {
            lazy,
            name,
            abcfile,
        })
    }

    fn write(&self, stream: &mut StreamWriter, _movie: &Movie) -> Result<()> {
        stream.write_u32(self.lazy as u32)?;
        stream.write_null_string(&self.name)?;
        self.abcfile.write(stream)?;
        Ok(())
    }
}
