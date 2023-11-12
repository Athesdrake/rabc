use super::Trait;
use crate::{error::Result, StreamReader, StreamWriter};

#[derive(PartialEq, Debug)]
pub struct Script {
    init: u32,
    traits: Vec<Trait>,
}

impl Script {
    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        let init = stream.read_u30()?;
        let count = stream.read_u30()?;
        let mut traits = Vec::with_capacity(count as usize);
        for _ in 0..count {
            traits.push(Trait::read(stream)?);
        }

        Ok(Self { init, traits })
    }

    pub(crate) fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u30(self.init)?;
        stream.write_u30(self.traits.len() as u32)?;
        for t in &self.traits {
            t.write(stream)?;
        }
        Ok(())
    }
}
