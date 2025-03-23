use std::collections::HashMap;

use super::ITag;
use crate::{error::Result, Movie, StreamReader, StreamWriter};

#[derive(Clone, Debug, PartialEq)]
pub struct SymbolClassTag {
    /// ignored when writing. Use Movie.symbols instead
    pub symbols: HashMap<u16, String>,
}

impl ITag for SymbolClassTag {
    fn read(stream: &mut StreamReader) -> Result<Self> {
        let mut symbols: HashMap<u16, String> = HashMap::new();
        for _ in 0..stream.read_u16()? {
            symbols.insert(stream.read_u16()?, stream.read_null_string()?);
        }
        Ok(Self { symbols })
    }

    fn write(&self, stream: &mut StreamWriter, movie: &Movie) -> Result<()> {
        stream.write_u16(self.symbols.len() as u16)?;
        for (id, name) in &movie.symbols {
            stream.write_u16(*id)?;
            stream.write_null_string(name)?;
        }
        Ok(())
    }
}
