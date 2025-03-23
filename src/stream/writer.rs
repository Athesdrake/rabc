use crate::error::Result;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Write;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct StreamWriter {
    buffer: Vec<u8>,
}

impl StreamWriter {
    pub fn new(buf: Vec<u8>) -> Self {
        Self { buffer: buf }
    }

    #[cfg(feature = "flate2")]
    pub fn deflate_zlib(&mut self, offset: usize, length: usize) -> Result<()> {
        use flate2::{write::ZlibEncoder, Compression};
        // Copy anything after the data we want to compress
        let mut after = Vec::with_capacity(self.len() - offset - length);
        after.clone_from_slice(&self.buffer[offset + length..]);

        let mut buf = Vec::with_capacity(length);
        let mut enc = ZlibEncoder::new(&mut buf, Compression::best());
        // Compress from `offset` and store the result into the temporary `buf`
        enc.write_all(&self.buffer[offset..])?;
        enc.finish()?;

        let end_pos = offset + buf.len();
        // Resize the buffer to the new size
        self.buffer.resize(end_pos + after.len(), 0);

        // Copy the compressed data into the buffer, while keeping the start & end of the buffer
        let out = &mut self.buffer;
        out[offset..end_pos].copy_from_slice(&buf);
        out[end_pos..].copy_from_slice(&after);
        Ok(())
    }
    #[cfg(not(feature = "flate2"))]
    pub fn deflate_zlib(&mut self, _offset: usize, _length: usize) -> Result<()> {
        use crate::error::RabcError;
        Err(Error::unsupported_compression("zlib"))
    }

    #[cfg(feature = "lzma-rs")]
    pub fn deflate_lzma(&mut self, offset: usize, length: usize) -> Result<()> {
        use lzma_rs::lzma_compress;
        use std::io::Cursor;
        // Copy anything after the data we want to compress
        let mut after = Vec::with_capacity(self.len() - offset - length);
        after.clone_from_slice(&self.buffer[offset + length..]);

        let mut buf = Vec::with_capacity(length);
        let mut cur = Cursor::new(&self.buffer[offset..offset + length]);
        lzma_compress(&mut cur, &mut buf)?;

        let end_pos = offset + buf.len() - 4;
        // Resize the buffer to the new size
        self.buffer.resize(end_pos + after.len(), 0);

        // Copy the compressed data into the buffer, while keeping the start & end of the buffer
        // Write the compressed size to respect the mangled header
        let mut size = Vec::with_capacity(4);
        let b = &mut self.buffer;
        size.write_u32::<LittleEndian>(buf.len() as u32 - 13)?;

        // Shitty mangled header ...
        b[offset..offset + 4].copy_from_slice(&size);
        b[offset + 4..offset + 9].copy_from_slice(&buf[0..5]);
        b[offset + 9..end_pos].copy_from_slice(&buf[13..]);
        b[end_pos..].copy_from_slice(&after);
        Ok(())
    }
    #[cfg(not(feature = "lzma-rs"))]
    pub fn deflate_lzma(&mut self, _offset: usize, _length: usize) -> Result<()> {
        use crate::error::RabcError;
        Err(Error::unsupported_compression("lzma"))
    }

    #[inline]
    pub fn buffer(&self) -> &Vec<u8> {
        &self.buffer
    }

    #[inline]
    pub fn move_buffer(self) -> Vec<u8> {
        self.buffer
    }

    #[inline]
    pub fn write_u8(&mut self, value: u8) -> Result<()> {
        Ok(self.buffer.write_u8(value)?)
    }
    #[inline]
    pub fn write_u16(&mut self, value: u16) -> Result<()> {
        Ok(self.buffer.write_u16::<LittleEndian>(value)?)
    }
    #[inline]
    pub fn write_u24(&mut self, value: u32) -> Result<()> {
        Ok(self.buffer.write_u24::<LittleEndian>(value)?)
    }
    #[inline]
    pub fn write_u32(&mut self, value: u32) -> Result<()> {
        Ok(self.buffer.write_u32::<LittleEndian>(value)?)
    }
    #[inline]
    pub fn write_u64(&mut self, value: u64) -> Result<()> {
        Ok(self.buffer.write_u64::<LittleEndian>(value)?)
    }
    #[inline]
    pub fn write_i8(&mut self, value: i8) -> Result<()> {
        Ok(self.buffer.write_i8(value)?)
    }
    #[inline]
    pub fn write_i16(&mut self, value: i16) -> Result<()> {
        Ok(self.buffer.write_i16::<LittleEndian>(value)?)
    }
    #[inline]
    pub fn write_i24(&mut self, value: i32) -> Result<()> {
        Ok(self.buffer.write_i24::<LittleEndian>(value)?)
    }
    #[inline]
    pub fn write_i32(&mut self, value: i32) -> Result<()> {
        Ok(self.buffer.write_i32::<LittleEndian>(value)?)
    }
    #[inline]
    pub fn write_i64(&mut self, value: i64) -> Result<()> {
        Ok(self.buffer.write_i64::<LittleEndian>(value)?)
    }
    #[inline]
    pub fn write_float(&mut self, value: f32) -> Result<()> {
        Ok(self.buffer.write_f32::<LittleEndian>(value)?)
    }
    #[inline]
    pub fn write_double(&mut self, value: f64) -> Result<()> {
        Ok(self.buffer.write_f64::<LittleEndian>(value)?)
    }

    // Read a variable-length unsigned integer. See https://en.wikipedia.org/wiki/LEB128 for more informations.
    #[inline]
    pub fn write_u30(&mut self, mut value: u32) -> Result<()> {
        loop {
            let byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                self.buffer.push(byte | 0x80);
            } else {
                self.buffer.push(byte);
                break;
            }
        }
        Ok(())
    }
    // Read a variable-length signed integer. See https://en.wikipedia.org/wiki/LEB128 for more informations.
    #[inline]
    pub fn write_i30(&mut self, value: i32) -> Result<()> {
        self.write_u30(value as u32)
    }

    #[inline]
    pub fn write_exact(&mut self, buf: &[u8]) -> Result<()> {
        self.buffer.write_all(buf)?;
        Ok(())
    }

    #[inline]
    pub fn write_null_string(&mut self, value: &String) -> Result<()> {
        self.write_exact(value.as_bytes())?;
        self.buffer.push(0);
        Ok(())
    }
    #[inline]
    pub fn write_string(&mut self, value: &String) -> Result<()> {
        // This is quite slow, we copy the value to a new string, but also because rust validate utf-8
        self.write_u30(value.len() as u32)?;
        self.write_exact(value.as_bytes())?;
        Ok(())
    }

    #[inline]
    pub fn write_stream(&mut self, value: &StreamWriter) -> Result<()> {
        self.write_exact(&value.buffer)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.buffer.len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn copy(&self) -> Result<Self> {
        Ok(StreamWriter::new(self.buffer.clone()))
    }

    #[inline]
    pub fn to_file(&self, mut file: std::fs::File) -> Result<()> {
        Ok(file.write_all(&self.buffer)?)
    }

    #[inline]
    pub(crate) fn write_u32_at(&mut self, value: u32, offset: usize) -> Result<()> {
        let mut buf = &mut self.buffer[offset..offset + 4];
        buf.write_u32::<LittleEndian>(value)?;
        Ok(())
    }
}

impl Write for StreamWriter {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, std::io::Error> {
        self.buffer.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }
}

#[cfg(test)]
mod tests {
    use crate::StreamWriter;

    #[test]
    pub fn new_stream() {
        let stream: StreamWriter = StreamWriter::new(vec![]);
        assert_eq!(stream.buffer.len(), 0);
    }
    #[test]
    pub fn test_write_u8() {
        let mut stream: StreamWriter = Default::default();
        stream.write_u8(0x7f).unwrap();
        assert_eq!(stream.buffer, vec![0x7f]);
    }
    #[test]
    pub fn test_write_u16() {
        let mut stream: StreamWriter = Default::default();
        stream.write_u16(0x0102).unwrap();
        assert_eq!(stream.buffer, vec![2, 1]);
    }
    #[test]
    pub fn test_write_u24() {
        let mut stream: StreamWriter = Default::default();
        stream.write_u24(0x010203).unwrap();
        assert_eq!(stream.buffer, vec![3, 2, 1]);
    }
    #[test]
    pub fn test_write_u32() {
        let mut stream: StreamWriter = Default::default();
        stream.write_u32(0x01020304).unwrap();
        assert_eq!(stream.buffer, vec![4, 3, 2, 1]);
    }
    #[test]
    pub fn test_write_u64() {
        let mut stream: StreamWriter = Default::default();
        stream.write_u64(0x0102030405060708).unwrap();
        assert_eq!(stream.buffer, vec![8, 7, 6, 5, 4, 3, 2, 1]);
    }
    #[test]
    pub fn test_write_i8() {
        let mut stream: StreamWriter = Default::default();
        stream.write_i8(-69).unwrap();
        assert_eq!(stream.buffer, vec![187]);
    }
    #[test]
    pub fn test_write_i16() {
        let mut stream: StreamWriter = Default::default();
        stream.write_i16(-6969).unwrap();
        assert_eq!(stream.buffer, vec![199, 228]);
    }
    #[test]
    pub fn test_write_i24() {
        let mut stream: StreamWriter = Default::default();
        stream.write_i24(-696969).unwrap();
        assert_eq!(stream.buffer, vec![119, 93, 245]);
    }
    #[test]
    pub fn test_write_i32() {
        let mut stream: StreamWriter = Default::default();
        stream.write_i32(-69696969).unwrap();
        assert_eq!(stream.buffer, vec![55, 130, 216, 251]);
    }
    #[test]
    pub fn test_write_i64() {
        let mut stream: StreamWriter = Default::default();
        stream.write_i64(-0x123456789012345).unwrap();
        assert_eq!(stream.buffer, vec![187, 220, 254, 118, 152, 186, 220, 254]);
    }
    #[test]
    pub fn test_write_float() {
        let mut stream: StreamWriter = Default::default();
        stream.write_float(-69.42).unwrap();
        assert_eq!(stream.buffer, vec![10, 215, 138, 194]);
    }
    #[test]
    pub fn test_write_double() {
        let mut stream: StreamWriter = Default::default();
        stream.write_double(-69.42).unwrap();
        assert_eq!(stream.buffer, vec![123, 20, 174, 71, 225, 90, 81, 192]);
    }
    #[test]
    pub fn test_write_u30() {
        let mut stream: StreamWriter = Default::default();
        stream.write_u30(69420).unwrap();
        assert_eq!(stream.buffer, vec![172, 158, 4]);
    }
    #[test]
    pub fn test_write_i30() {
        let mut stream: StreamWriter = Default::default();
        stream.write_i30(-69420).unwrap();
        assert_eq!(stream.buffer, vec![212, 225, 251, 255, 15]);
    }
}
