use crate::error::{RabcError, Result};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{BufRead, Cursor, Read, Seek, SeekFrom};

use super::StreamWriter;

#[derive(Debug, PartialEq)]
pub struct StreamReader<'a> {
    pub(crate) buffer: Cursor<&'a [u8]>,
}

impl<'a> StreamReader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            buffer: Cursor::new(buf),
        }
    }

    #[cfg(feature = "flate2")]
    pub fn inflate_zlib(&mut self, capacity: usize) -> Result<Vec<u8>> {
        use flate2::read::ZlibDecoder;

        // Create our output buffer
        let mut output = Vec::with_capacity(capacity);
        let pos = self.buffer.position() as usize;
        let mut decoder = ZlibDecoder::new(&self.buffer.get_ref()[pos..]);

        // decompress!
        decoder
            .read_to_end(&mut output)
            .map_err(|e| RabcError::InvalidDeflateStream(e.to_string()))?;

        Ok(output)
    }
    #[cfg(not(feature = "flate2"))]
    pub fn inflate_zlib(&mut self, _capacity: usize) -> Result<Vec<u8>> {
        Err(RabcError::unsupported_compression("zlib"))
    }

    #[cfg(feature = "lzma-rs")]
    pub fn inflate_lzma(&mut self, capacity: usize) -> Result<Vec<u8>> {
        use lzma_rs::{
            decompress::{Options, UnpackedSize::UseProvided},
            lzma_decompress_with_options,
        };
        // lzma compressed swf has a mangled header.
        // Skip 4 bytes (half of the header) and provide the stream's size to lmza-rs
        self.skip(4)?;

        // Create our output buffer
        let mut output = Vec::with_capacity(capacity);

        // decompress!
        lzma_decompress_with_options(
            &mut self.buffer,
            &mut output,
            &Options {
                unpacked_size: UseProvided(Some(capacity as u64)),
                allow_incomplete: true,
                memlimit: None,
            },
        )
        .map_err(|e| RabcError::InvalidLzmaStream(e.to_string()))?;

        Ok(output)
    }
    #[cfg(not(feature = "lzma-rs"))]
    pub fn inflate_lzma(&mut self, _capacity: usize) -> Result<Vec<u8>> {
        Err(RabcError::unsupported_compression("lzma"))
    }

    #[inline]
    pub fn read_u8(&mut self) -> Result<u8> {
        Ok(self.buffer.read_u8()?)
    }
    #[inline]
    pub fn read_u16(&mut self) -> Result<u16> {
        Ok(self.buffer.read_u16::<LittleEndian>()?)
    }
    #[inline]
    pub fn read_u24(&mut self) -> Result<u32> {
        Ok(self.buffer.read_u24::<LittleEndian>()?)
    }
    #[inline]
    pub fn read_u32(&mut self) -> Result<u32> {
        Ok(self.buffer.read_u32::<LittleEndian>()?)
    }
    #[inline]
    pub fn read_u64(&mut self) -> Result<u64> {
        Ok(self.buffer.read_u64::<LittleEndian>()?)
    }
    #[inline]
    pub fn read_i8(&mut self) -> Result<i8> {
        Ok(self.buffer.read_i8()?)
    }
    #[inline]
    pub fn read_i16(&mut self) -> Result<i16> {
        Ok(self.buffer.read_i16::<LittleEndian>()?)
    }
    #[inline]
    pub fn read_i24(&mut self) -> Result<i32> {
        Ok(self.buffer.read_i24::<LittleEndian>()?)
    }
    #[inline]
    pub fn read_i32(&mut self) -> Result<i32> {
        Ok(self.buffer.read_i32::<LittleEndian>()?)
    }
    #[inline]
    pub fn read_i64(&mut self) -> Result<i64> {
        Ok(self.buffer.read_i64::<LittleEndian>()?)
    }
    #[inline]
    pub fn read_float(&mut self) -> Result<f32> {
        Ok(self.buffer.read_f32::<LittleEndian>()?)
    }
    #[inline]
    pub fn read_double(&mut self) -> Result<f64> {
        Ok(self.buffer.read_f64::<LittleEndian>()?)
    }

    // Read a variable-length unsigned integer. See https://en.wikipedia.org/wiki/LEB128 for more informations.
    #[inline]
    pub fn read_u30(&mut self) -> Result<u32> {
        let mut value: u32 = 0;

        for i in (0..35).step_by(7) {
            let byte = self.buffer.read_u8()? as u32;
            value += (byte & 0x7f) << i;
            if byte & 0x80 == 0 {
                break;
            }
        }
        Ok(value)
    }
    // Read a variable-length signed integer. See https://en.wikipedia.org/wiki/LEB128 for more informations.
    #[inline]
    pub fn read_i30(&mut self) -> Result<i32> {
        Ok(self.read_u30()? as i32)
    }

    #[inline]
    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        Ok(self.buffer.read_exact(buf)?)
    }

    #[inline]
    pub fn skip(&mut self, length: u32) -> Result<u32> {
        Ok(self.buffer.seek(SeekFrom::Current(length.into()))? as u32)
    }

    #[inline]
    pub fn read_null_string(&mut self) -> Result<String> {
        let mut buf = Vec::new();
        self.buffer.read_until(0, &mut buf)?;
        buf.pop();
        Ok(String::from_utf8(buf)?)
    }
    #[inline]
    pub fn read_string(&mut self) -> Result<String> {
        // This is quite slow, we copy the value to a new string, but also because rust validate utf-8
        let length = self.read_u30()? as usize;
        let mut buf = vec![0u8; length];
        self.buffer.read_exact(buf.as_mut())?;
        Ok(String::from_utf8(buf)?)
    }

    #[inline]
    pub fn pos(&self) -> u32 {
        self.buffer.position() as u32
    }
    #[inline]
    pub fn len(&self) -> u32 {
        self.buffer.get_ref().len() as u32
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[inline]
    pub fn remaining(&self) -> u32 {
        self.len() - self.buffer.position() as u32
    }
    #[inline]
    pub fn finished(&self) -> bool {
        self.remaining() == 0
    }

    pub fn copy(&self) -> Result<Self> {
        let mut stream = StreamReader::new(self.buffer.get_ref());
        stream.skip(self.buffer.position() as u32)?;
        Ok(stream)
    }

    #[inline]
    pub fn write_to_stream(&self, stream: &mut StreamWriter) -> Result<()> {
        stream.write_exact(self.buffer.get_ref())
    }
}

#[cfg(test)]
mod tests {
    use crate::StreamReader;

    #[test]
    pub fn new_stream() {
        let stream = StreamReader::new(&[]);
        assert_eq!(stream.buffer.get_ref().len(), 0);
    }
    #[test]
    pub fn test_read_u8() {
        let mut stream = StreamReader::new(&[0x7f]);
        assert_eq!(stream.read_u8().unwrap(), 0x7f);
    }
    #[test]
    pub fn test_read_u16() {
        let mut stream = StreamReader::new(&[2, 1]);
        assert_eq!(stream.read_u16().unwrap(), 0x0102);
    }
    #[test]
    pub fn test_read_u24() {
        let mut stream = StreamReader::new(&[3, 2, 1]);
        assert_eq!(stream.read_u24().unwrap(), 0x010203);
    }
    #[test]
    pub fn test_read_u32() {
        let mut stream = StreamReader::new(&[4, 3, 2, 1]);
        assert_eq!(stream.read_u32().unwrap(), 0x01020304);
    }
    #[test]
    pub fn test_read_u64() {
        let mut stream = StreamReader::new(&[8, 7, 6, 5, 4, 3, 2, 1]);
        assert_eq!(stream.read_u64().unwrap(), 0x0102030405060708);
    }
    #[test]
    pub fn test_read_i8() {
        let mut stream = StreamReader::new(&[187]);
        assert_eq!(stream.read_i8().unwrap(), -69);
    }
    #[test]
    pub fn test_read_i16() {
        let mut stream = StreamReader::new(&[199, 228]);
        assert_eq!(stream.read_i16().unwrap(), -6969);
    }
    #[test]
    pub fn test_read_i24() {
        let mut stream = StreamReader::new(&[119, 93, 245]);
        assert_eq!(stream.read_i24().unwrap(), -696969);
    }
    #[test]
    pub fn test_read_i32() {
        let mut stream = StreamReader::new(&[55, 130, 216, 251]);
        assert_eq!(stream.read_i32().unwrap(), -69696969);
    }
    #[test]
    pub fn test_read_i64() {
        let mut stream = StreamReader::new(&[187, 220, 254, 118, 152, 186, 220, 254]);
        assert_eq!(stream.read_i64().unwrap(), -0x123456789012345);
    }
    #[test]
    pub fn test_read_float() {
        let mut stream = StreamReader::new(&[10, 215, 138, 194]);
        assert_eq!(stream.read_float().unwrap(), -69.42);
    }
    #[test]
    pub fn test_read_double() {
        let mut stream = StreamReader::new(&[123, 20, 174, 71, 225, 90, 81, 192]);
        assert_eq!(stream.read_double().unwrap(), -69.42);
    }
    #[test]
    pub fn test_read_u30() {
        let mut stream = StreamReader::new(&[172, 158, 4]);
        assert_eq!(stream.read_u30().unwrap(), 69420);
    }
    #[test]
    pub fn test_read_i30() {
        let mut stream = StreamReader::new(&[212, 225, 251, 255, 127]);
        assert_eq!(stream.read_i30().unwrap(), -69420);
    }
    #[test]
    pub fn test_read_too_much() {
        let mut stream = StreamReader::new(&[42]);
        assert!(stream.read_u16().is_err());
    }
    #[test]
    pub fn test_read_u30_too_much() {
        let mut stream = StreamReader::new(&[0x80, 0x80, 0x80, 0x80]);
        assert!(stream.read_u30().is_err());
    }
}
