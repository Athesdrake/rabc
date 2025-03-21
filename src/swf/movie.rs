use crate::{
    error::{RabcError, Result},
    swf::{datatypes::Rect, tags::*},
    StreamReader, StreamWriter,
};
use std::{collections::HashMap, fmt};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum Compression {
    #[default]
    None,
    Zlib,
    Lzma,
}

#[derive(Debug)]
pub struct Movie {
    pub compression: Compression,
    pub version: u8,
    pub file_length: u32,
    pub framerate: f64,
    pub framecount: u16,
    pub framesize: Rect,

    pub tags: Vec<Tag>,
    pub symbols: HashMap<u16, String>,
}

impl Default for Movie {
    fn default() -> Self {
        Self {
            compression: Default::default(),
            version: 14,
            file_length: Default::default(),
            framerate: Default::default(),
            framecount: Default::default(),
            framesize: Default::default(),
            tags: Default::default(),
            symbols: Default::default(),
        }
    }
}

pub struct Header {
    pub compression: Compression,
    pub version: u8,
    pub file_length: u32,
}

impl Header {
    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        let mut signature = [0u8; 3];
        stream.read_exact(&mut signature)?;
        if &signature[1..3] != b"WS" {
            return Err(RabcError::InvalidSignature(
                signature
                    .iter()
                    .map(|&b| {
                        char::from_u32(b.into())
                            .map(|c| c.to_string())
                            .unwrap_or_else(|| format!("\\x{:02x}", b))
                    })
                    .collect(),
            ));
        }

        let compression = is_valid_compression(signature[0])?;
        let version = stream.read_u8()?;
        let file_length = stream.read_u32()?;

        Ok(Self {
            compression,
            version,
            file_length,
        })
    }

    pub fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u8(match self.compression {
            Compression::None => b'F',
            Compression::Zlib => b'C',
            Compression::Lzma => b'Z',
        })?;
        stream.write_exact(b"WS")?;
        stream.write_u8(self.version)?;
        // Don't write the file's length now, as we don't know yet the size
        stream.write_u32(0)?;
        Ok(())
    }
}

impl Movie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read(mut stream: StreamReader) -> Result<Self> {
        let header = Header::read(&mut stream)?;
        let size = header.file_length as usize;
        let buffer = match header.compression {
            Compression::Zlib => Some(stream.inflate_zlib(size - 8)?),
            Compression::Lzma => Some(stream.inflate_lzma(size - 8)?),
            Compression::None => None,
        };
        let mut stream = match &buffer {
            Some(buffer) => StreamReader::new(buffer),
            None => stream,
        };

        let framesize = Rect::read(&mut stream)?;
        let framerate = (stream.read_u8()? as f64) / 256.0 + (stream.read_u8()? as f64);
        let framecount = stream.read_u16()?;
        let mut tags: Vec<Tag> = Vec::new();
        let mut tag_type = TagID::Unknown;
        let mut symbols = HashMap::new();

        while tag_type != TagID::End {
            let hdr = stream.read_u16()?;
            let tag_id = hdr >> 6;
            tag_type = TagID::from_u16(tag_id);
            let length: u32 = match hdr & 0x3F {
                0x3F => stream.read_u32()?,
                _ => (hdr & 0x3F).into(),
            };

            let pos = stream.pos();
            stream.skip(length)?;
            let data = &stream.buffer.get_ref()[pos as usize..(pos + length) as usize];
            let mut ts = StreamReader::new(data);

            if tag_type == TagID::Unknown {
                tags.push(Tag::Unknown(UnknownTag::read_with_id(data, tag_id)?));
            } else {
                let tag = Tag::read(tag_type, &mut ts)?;
                if let Tag::SymbolClass(t) = &tag {
                    t.symbols.clone_into(&mut symbols);
                }
                tags.push(tag);
                assert_eq!(
                    ts.remaining(),
                    0,
                    "Remaining data after tag 0x{:0>2x}.",
                    tag_id
                );
            }
        }

        Ok(Self {
            compression: header.compression,
            version: header.version,
            file_length: header.file_length,
            framerate,
            framecount,
            framesize,
            tags,
            symbols,
        })
    }

    pub fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        let header = Header {
            compression: self.compression,
            version: self.version,
            file_length: 0,
        };

        header.write(stream)?;
        self.framesize.write(stream)?;

        let integral = self.framerate.ceil();
        let fraction = self.framerate - integral;
        stream.write_u8((fraction * 256.0) as u8)?;
        stream.write_u8(integral as u8)?;
        stream.write_u16(self.framecount)?;

        for tag in &self.tags {
            let mut stag = StreamWriter::new(Vec::new());
            tag.write(&mut stag, self)?;

            let length = stag.len();
            let id = tag.id();

            if length < 0x3F {
                stream.write_u16(id << 6 | (length & 0x3F) as u16)?;
            } else {
                stream.write_u16(id << 6 | 0x3F)?;
                stream.write_u32(length as u32)?;
            }
            stream.write_stream(&stag)?;
        }

        stream.write_u32_at(stream.len() as u32, 4)?;
        match self.compression {
            Compression::Zlib => stream.deflate_zlib(8, stream.len() - 8)?,
            Compression::Lzma => stream.deflate_lzma(8, stream.len() - 8)?,
            Compression::None => {}
        };
        Ok(())
    }

    pub fn frame1(&self) -> Option<&DoABCTag> {
        for tag in &self.tags {
            if let Tag::DoABC(doabc) = tag {
                if doabc.name == "frame1" {
                    return Some(doabc);
                }
            }
        }
        None
    }
    pub fn frame1_mut(&'_ mut self) -> Option<&'_ mut DoABCTag> {
        for tag in &mut self.tags {
            if let Tag::DoABC(doabc) = tag {
                if doabc.name == "frame1" {
                    return Some(doabc);
                }
            }
        }
        None
    }

    pub fn binaries(&self) -> impl Iterator<Item = &'_ DefineBinaryDataTag> {
        self.tags.iter().filter_map(|t| match t {
            Tag::DefineBinaryData(t) => Some(t),
            _ => None,
        })
    }
    pub fn binaries_mut(&mut self) -> impl Iterator<Item = &'_ mut DefineBinaryDataTag> {
        self.tags.iter_mut().filter_map(|t| match t {
            Tag::DefineBinaryData(t) => Some(t),
            _ => None,
        })
    }
}

fn is_valid_compression(signature: u8) -> Result<Compression> {
    match signature {
        b'F' => Ok(Compression::None),
        b'C' => Ok(Compression::Zlib),
        b'Z' => Ok(Compression::Lzma),
        b => Err(RabcError::InvalidCompression(char::from(b))),
    }
}

impl fmt::Display for Compression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Compression::{}",
            match self {
                Compression::None => "None",
                Compression::Zlib => "Zlib",
                Compression::Lzma => "Lzma",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{is_valid_compression, Compression, Header};
    use crate::StreamReader;

    #[test]
    pub fn valid_compression() {
        assert_eq!(is_valid_compression(b'F').unwrap(), Compression::None);
        assert_eq!(is_valid_compression(b'C').unwrap(), Compression::Zlib);
        assert_eq!(is_valid_compression(b'Z').unwrap(), Compression::Lzma);
        is_valid_compression(b'W').unwrap_err();
    }

    #[test]
    pub fn read_header() {
        let mut stream = StreamReader::new(b"FWS\x0e,\x0f\x01\x00");
        let header = Header::read(&mut stream).unwrap();
        assert_eq!(header.compression, Compression::None);
        assert_eq!(header.version, 14);
        assert_eq!(header.file_length, 69420);
    }
}
