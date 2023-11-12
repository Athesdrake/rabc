use super::ITag;
use crate::{error::Result, Movie, StreamReader, StreamWriter};

#[derive(Debug, PartialEq)]
pub struct ScriptLimitsTag {
    /// Set the max recursion limit. Default is 256
    pub max_recursion_depth: u16,
    /// Set the maximum number of seconds the player should process ActionScript before asking if the script should be
    /// stopped. The default value varies by platform and is between 15 and 20 seconds.
    pub script_timeout_seconds: u16,
}

impl ScriptLimitsTag {
    pub fn new() -> Self {
        Self {
            max_recursion_depth: 256,
            script_timeout_seconds: 20,
        }
    }
}

impl ITag for ScriptLimitsTag {
    fn read(stream: &mut StreamReader) -> Result<Self> {
        Ok(Self {
            max_recursion_depth: stream.read_u16()?,
            script_timeout_seconds: stream.read_u16()?,
        })
    }

    fn write(&self, stream: &mut StreamWriter, _movie: &Movie) -> Result<()> {
        stream.write_u16(self.max_recursion_depth)?;
        stream.write_u16(self.script_timeout_seconds)?;
        Ok(())
    }
}
