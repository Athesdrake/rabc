use crate::error::Result;
use crate::StreamReader;
use crate::StreamWriter;
use std::cmp::min;

#[derive(Debug)]
pub struct BitStreamReader<'a, 'b> {
    stream: &'a mut StreamReader<'b>,
    current_byte: u8,
    bitpos: u8,
}

impl<'a, 'b> BitStreamReader<'a, 'b> {
    pub fn new(stream: &'a mut StreamReader<'b>) -> Self {
        Self {
            stream,
            current_byte: 0,
            bitpos: 8,
        }
    }

    pub fn read_ub(&mut self, n_bits: u8) -> Result<u32> {
        if n_bits == 0 {
            return Ok(0);
        }
        if self.bitpos > 7 {
            self.bitpos = 0;
            self.current_byte = self.stream.read_u8()?;
        }
        // read the maximum of bits available
        let start_bits = min(8 - self.bitpos, n_bits);
        let value: u32 = ((self.current_byte as u32) >> (8 - start_bits - self.bitpos))
            & ((1 << start_bits) - 1);

        // increment the position by the bits read
        self.bitpos += start_bits;

        // read more bits if needed
        let left_bits = n_bits - start_bits;
        if left_bits > 0 {
            return Ok((value << left_bits) | self.read_ub(left_bits)?);
        }
        Ok(value)
    }

    pub fn read_sb(&mut self, n_bits: u8) -> Result<i32> {
        if n_bits < 2 {
            return Ok(self.read_ub(n_bits)? as i32);
        }

        let value = self.read_ub(n_bits)? as i32;
        let shift = 32 - n_bits;

        // Shift to retrieve the value's sign
        Ok((value << shift) >> shift)
    }
}

#[derive(Debug)]
pub struct BitStreamWriter<'a> {
    stream: &'a mut StreamWriter,
    current_byte: u8,
    bitpos: u8,
}

impl<'a> BitStreamWriter<'a> {
    pub fn new(stream: &'a mut StreamWriter) -> Self {
        Self {
            stream,
            current_byte: 0,
            bitpos: 0,
        }
    }

    pub fn write_ub(&mut self, mut n_bits: u8, mut value: u32) -> Result<()> {
        while n_bits > 0 {
            let maxbits = min(8 - self.bitpos, n_bits);
            let bits = (((value >> (n_bits - maxbits)) & ((1 << maxbits) - 1))
                << (8 - maxbits - self.bitpos)) as u8;

            self.current_byte |= bits;
            self.bitpos += maxbits;
            n_bits -= maxbits;
            if self.bitpos > 7 {
                self.bitpos = 0;
                self.stream.write_u8(self.current_byte)?;
                self.current_byte = 0;
            }

            if n_bits > 0 {
                value &= (1 << n_bits) - 1;
            }
        }
        Ok(())
    }

    pub fn write_sb(&mut self, n_bits: u8, value: i32) -> Result<()> {
        if n_bits < 2 {
            Ok(())
        } else {
            let mut val = value as u32;
            if value < 0 {
                val &= !(u32::MAX << n_bits);
            }
            self.write_ub(n_bits, val)
        }
    }

    pub fn flush(&mut self) -> Result<()> {
        if self.bitpos != 0 {
            self.bitpos = 0;
            self.stream.write_u8(self.current_byte)?;
        }
        Ok(())
    }

    #[inline]
    pub fn calc_ubits(value: u32) -> u8 {
        if value == 0 {
            return 0;
        }
        (u32::BITS - (value - 1).leading_zeros()) as u8
    }
    #[inline]
    pub fn calc_sbits(value: i32) -> u8 {
        Self::calc_ubits(value.unsigned_abs()) + 1
    }
}

impl Drop for BitStreamWriter<'_> {
    fn drop(&mut self) {
        if self.bitpos != 0 {
            panic!("BitStreamWriter was not flushed.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BitStreamWriter;

    fn calc_ubits_math(value: u32) -> u8 {
        (value as f64).log2().ceil() as u8
    }
    fn calc_sbits_math(value: i32) -> u8 {
        calc_ubits_math(value.unsigned_abs()) + 1
    }

    #[test]
    fn test_calc_ubits() {
        for i in 2..100_000 {
            assert_eq!(
                BitStreamWriter::calc_ubits(i),
                calc_ubits_math(i),
                "incorrect bit size for {i}"
            );
        }
    }
    #[test]
    fn test_calc_sbits() {
        for i in -50_000..-2 {
            assert_eq!(
                BitStreamWriter::calc_sbits(i),
                calc_sbits_math(i),
                "incorrect bit size for {i}"
            );
        }
        for i in 2..50_000 {
            assert_eq!(
                BitStreamWriter::calc_sbits(i),
                calc_sbits_math(i),
                "incorrect bit size for {i}"
            );
        }
    }
}
