use super::ITag;
use crate::{error::Result, Movie, StreamReader, StreamWriter};
use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct FileAttributes : u8 {
        const USE_NETWORK = 0x01;
        const ACTION_SCIPT3 = 0x08;
        const HAS_METADATA = 0x10;
        const USE_GPU = 0x20;
        const USE_DIRECT_BLIT = 0x40;
    }
}

#[derive(Debug, PartialEq)]
pub struct FileAttributesTag {
    flags: FileAttributes,
}

impl FileAttributesTag {
    /// Tells the Flash Player to use hardware acceleration to blit graphics to the screen if available
    pub fn use_direct_blit(&self) -> bool {
        self.flags.contains(FileAttributes::USE_DIRECT_BLIT)
    }
    /// Tells the Player to use GPU compositing features if available
    pub fn use_gpu(&self) -> bool {
        self.flags.contains(FileAttributes::USE_GPU)
    }
    /// Defines if the SWF has metadata
    pub fn has_metadata(&self) -> bool {
        self.flags.contains(FileAttributes::HAS_METADATA)
    }
    /// Defines if the SWF contains AVM2 code
    pub fn use_as3(&self) -> bool {
        self.flags.contains(FileAttributes::ACTION_SCIPT3)
    }
    /// Gives network access if the file is run on a local drive
    pub fn use_network(&self) -> bool {
        self.flags.contains(FileAttributes::USE_NETWORK)
    }

    /// Tells the Flash Player to use hardware acceleration to blit graphics to the screen if available
    pub fn set_use_direct_blit(&mut self, enable: bool) {
        self.flags.set(FileAttributes::USE_DIRECT_BLIT, enable);
    }
    /// Tells the Player to use GPU compositing features if available
    pub fn set_use_gpu(&mut self, enable: bool) {
        self.flags.set(FileAttributes::USE_GPU, enable);
    }
    /// Defines if the SWF has metadata
    pub fn set_has_metadata(&mut self, enable: bool) {
        self.flags.set(FileAttributes::HAS_METADATA, enable);
    }
    /// Defines if the SWF contains AVM2 code
    pub fn set_use_as3(&mut self, enable: bool) {
        self.flags.set(FileAttributes::ACTION_SCIPT3, enable);
    }
    /// Gives network access if the file is run on a local drive
    pub fn set_use_network(&mut self, enable: bool) {
        self.flags.set(FileAttributes::USE_NETWORK, enable);
    }
}

impl ITag for FileAttributesTag {
    fn read(stream: &mut StreamReader) -> Result<Self> {
        let flags = FileAttributes::from_bits_retain(stream.read_u8()?);
        stream.read_i24()?;
        Ok(Self { flags })
    }

    fn write(&self, stream: &mut StreamWriter, _movie: &Movie) -> Result<()> {
        stream.write_u8(self.flags.bits())?;
        stream.write_u24(0)?;
        Ok(())
    }
}
