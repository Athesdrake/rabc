use super::ITag;
use crate::{error::Result, Movie, StreamReader, StreamWriter};

#[derive(Clone, Debug, PartialEq)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProductInfoTag {
    pub product_id: u32,
    pub edition: u32,
    pub version: Version,
    pub build: u64,
    pub compile_date: u64,
}

impl ITag for ProductInfoTag {
    fn read(stream: &mut StreamReader) -> Result<Self> {
        Ok(Self {
            product_id: stream.read_u32()?,
            edition: stream.read_u32()?,
            version: Version {
                major: stream.read_u8()?,
                minor: stream.read_u8()?,
            },
            build: stream.read_u64()?,
            compile_date: stream.read_u64()?,
        })
    }

    fn write(&self, stream: &mut StreamWriter, _movie: &Movie) -> Result<()> {
        stream.write_u32(self.product_id)?;
        stream.write_u32(self.edition)?;
        stream.write_u8(self.version.major)?;
        stream.write_u8(self.version.minor)?;
        stream.write_u64(self.build)?;
        stream.write_u64(self.compile_date)?;
        Ok(())
    }
}
