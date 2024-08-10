use crate::error::Result;
use crate::stream::{BitStreamReader, BitStreamWriter, StreamReader, StreamWriter};
use std::fmt;

const TWIPS: i32 = 20;

#[derive(Debug, PartialEq, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq, Default)]
pub struct Rect {
    pub min: Position,
    pub max: Position,
}

#[derive(Debug, PartialEq, Default)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, PartialEq, Default)]
pub struct Rgba {
    pub rgb: Rgb,
    pub a: u8,
}

impl Rect {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        let mut bs = BitStreamReader::new(stream);
        let mut min = Position { x: 0, y: 0 };
        let mut max = Position { x: 0, y: 0 };

        let n_bits = bs.read_ub(5)? as u8;
        min.x = bs.read_sb(n_bits)?;
        max.x = bs.read_sb(n_bits)?;
        min.y = bs.read_sb(n_bits)?;
        max.y = bs.read_sb(n_bits)?;
        Ok(Self { min, max })
    }
    pub fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        let mut bs = BitStreamWriter::new(stream);

        let n_bits = *[self.min.x, self.min.y, self.max.x, self.max.y]
            .map(BitStreamWriter::calc_sbits)
            .iter()
            .max()
            .unwrap();

        bs.write_ub(5, n_bits.into())?;
        bs.write_sb(n_bits, self.min.x)?;
        bs.write_sb(n_bits, self.max.x)?;
        bs.write_sb(n_bits, self.min.y)?;
        bs.write_sb(n_bits, self.max.y)?;
        bs.flush()?;
        Ok(())
    }
}

impl Rgb {
    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        Ok(Self {
            r: stream.read_u8()?,
            g: stream.read_u8()?,
            b: stream.read_u8()?,
        })
    }

    pub fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_u8(self.r)?;
        stream.write_u8(self.g)?;
        stream.write_u8(self.b)?;
        Ok(())
    }
}
impl Rgba {
    #[allow(dead_code)]
    pub fn read(stream: &mut StreamReader) -> Result<Self> {
        let rgb = Rgb::read(stream)?;
        Ok(Self {
            rgb,
            a: stream.read_u8()?,
        })
    }

    #[allow(dead_code)]
    pub fn write(&self, stream: &mut StreamWriter) -> Result<()> {
        self.rgb.write(stream)?;
        stream.write_u8(self.a)
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[Rect x:{} y:{} width:{} height:{}]",
            self.min.x / TWIPS,
            self.min.y / TWIPS,
            self.max.x / TWIPS - self.min.x / TWIPS,
            self.max.y / TWIPS - self.min.y / TWIPS
        )
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGB({}, {}, {})", self.r, self.g, self.b)
    }
}

impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RGBA({}, {}, {}, {})",
            self.rgb.r, self.rgb.g, self.rgb.b, self.a
        )
    }
}
