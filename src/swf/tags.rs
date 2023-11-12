mod define_binary_data_tag;
mod do_abc_tag;
mod end_tag;
mod file_attributes_tag;
mod metadata_tag;
mod product_info_tag;
mod script_limits_tag;
mod set_background_color_tag;
mod symbol_class_tag;
mod unknown_tag;

use std::fmt::Display;

pub use define_binary_data_tag::DefineBinaryDataTag;
pub use do_abc_tag::DoABCTag;
pub use end_tag::EndTag;
pub use file_attributes_tag::{FileAttributes, FileAttributesTag};
pub use metadata_tag::MetadataTag;
pub use product_info_tag::ProductInfoTag;
pub use script_limits_tag::ScriptLimitsTag;
pub use set_background_color_tag::SetBackgroundColorTag;
pub use symbol_class_tag::SymbolClassTag;
pub use unknown_tag::UnknownTag;

use crate::{error::Result, Movie, StreamReader, StreamWriter};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagID {
    End = 0x00,
    SetBackgroundColor = 0x09,
    ProductInfo = 0x29,
    ScriptLimits = 0x41,
    FileAttributes = 0x45,
    SymbolClass = 0x4C,
    Metadata = 0x4D,
    DoABC = 0x52,
    DefineBinaryData = 0x57,
    Unknown = 0x3ff,
}

impl TagID {
    pub fn from_u16(id: u16) -> Self {
        match id {
            0x00 => Self::End,
            0x09 => Self::SetBackgroundColor,
            0x29 => Self::ProductInfo,
            0x41 => Self::ScriptLimits,
            0x45 => Self::FileAttributes,
            0x4C => Self::SymbolClass,
            0x4D => Self::Metadata,
            0x52 => Self::DoABC,
            0x57 => Self::DefineBinaryData,
            _ => Self::Unknown,
        }
    }

    /// Returns `true` if the tag id is Unknown.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

impl Display for TagID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::End => "EndTag",
                Self::SetBackgroundColor => "SetBackgroundColorTag",
                Self::ProductInfo => "ProductInfoTag",
                Self::ScriptLimits => "ScriptLimitsTag",
                Self::FileAttributes => "FileAttributesTag",
                Self::SymbolClass => "SymbolClassTag",
                Self::Metadata => "MetadataTag",
                Self::DoABC => "DoABCTag",
                Self::DefineBinaryData => "DefineBinaryDataTag",
                Self::Unknown => "UnknownTag",
            }
        )
    }
}

pub trait ITag {
    fn read(stream: &mut StreamReader) -> Result<Self>
    where
        Self: Sized;

    fn write(&self, stream: &mut StreamWriter, movie: &Movie) -> Result<()>;
}

#[derive(Debug, PartialEq)]
pub enum Tag {
    DefineBinaryData(DefineBinaryDataTag),
    End(EndTag),
    FileAttributes(FileAttributesTag),
    Metadata(MetadataTag),
    ProductInfo(ProductInfoTag),
    ScriptLimits(ScriptLimitsTag),
    SetBackgroundColor(SetBackgroundColorTag),
    SymbolClass(SymbolClassTag),
    DoABC(DoABCTag),
    Unknown(UnknownTag),
}

impl Tag {
    pub fn id(&self) -> u16 {
        self.into()
    }

    pub fn read(tag_type: TagID, stream: &mut StreamReader) -> Result<Self> {
        Ok(match tag_type {
            TagID::DefineBinaryData => Tag::DefineBinaryData(DefineBinaryDataTag::read(stream)?),
            TagID::DoABC => Tag::DoABC(DoABCTag::read(stream)?),
            TagID::End => Tag::End(EndTag::read(stream)?),
            TagID::FileAttributes => Tag::FileAttributes(FileAttributesTag::read(stream)?),
            TagID::Metadata => Tag::Metadata(MetadataTag::read(stream)?),
            TagID::ProductInfo => Tag::ProductInfo(ProductInfoTag::read(stream)?),
            TagID::ScriptLimits => Tag::ScriptLimits(ScriptLimitsTag::read(stream)?),
            TagID::SetBackgroundColor => {
                Tag::SetBackgroundColor(SetBackgroundColorTag::read(stream)?)
            }
            TagID::SymbolClass => Tag::SymbolClass(SymbolClassTag::read(stream)?),
            _ => unreachable!(),
        })
    }

    pub fn write(&self, stream: &mut StreamWriter, movie: &Movie) -> Result<()> {
        Ok(match self {
            Tag::DefineBinaryData(t) => t.write(stream, movie)?,
            Tag::DoABC(t) => t.write(stream, movie)?,
            Tag::End(t) => t.write(stream, movie)?,
            Tag::FileAttributes(t) => t.write(stream, movie)?,
            Tag::Metadata(t) => t.write(stream, movie)?,
            Tag::ProductInfo(t) => t.write(stream, movie)?,
            Tag::ScriptLimits(t) => t.write(stream, movie)?,
            Tag::SetBackgroundColor(t) => t.write(stream, movie)?,
            Tag::SymbolClass(t) => t.write(stream, movie)?,
            Tag::Unknown(t) => t.write(stream, movie)?,
        })
    }
}

impl From<&Tag> for TagID {
    fn from(tag: &Tag) -> Self {
        match tag {
            Tag::End(_) => Self::End,
            Tag::SetBackgroundColor(_) => Self::SetBackgroundColor,
            Tag::ProductInfo(_) => Self::ProductInfo,
            Tag::ScriptLimits(_) => Self::ScriptLimits,
            Tag::FileAttributes(_) => Self::FileAttributes,
            Tag::SymbolClass(_) => Self::SymbolClass,
            Tag::Metadata(_) => Self::Metadata,
            Tag::DoABC(_) => Self::DoABC,
            Tag::DefineBinaryData(_) => Self::DefineBinaryData,
            _ => Self::Unknown,
        }
    }
}
impl From<&Tag> for u16 {
    fn from(tag: &Tag) -> Self {
        match tag {
            Tag::Unknown(t) => t.id,
            _ => {
                let tag_id: TagID = tag.into();
                tag_id as u16
            }
        }
    }
}
