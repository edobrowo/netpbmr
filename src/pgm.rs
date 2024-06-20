//! PGM (Portable Gray Map) image encoding and decoding.
//!
//! Each PGM image fundamentally consists of the image width,
//! the image height, the bit depth, and a sequence of rows of
//! grey values. There are `height` number of rows, each with
//! `width` grey values.
//!
//! PGM `raw` files consist of a sequence of PGM images.
//! Grey values are serialized as unsigned binary integers.
//! The `raw` format uses the magic number `P5`.
//!
//! PGM `plain` files consist of a single PGM image.
//! Grey values are written as ASCII-encoded decimal numbers.
//! The `plain` format uses the magic number `P2`.

use crate::NetpbmError;
use crate::{formats::EncodingType, Info};
use std::io;

/// PGM encoder.
#[derive(Debug)]
pub struct Encoder<W: io::Write> {
    writer: W,
}

impl<W: io::Write> Encoder<W> {
    /// Create a new PGM encoder with the given writer.
    pub fn new(writer: W) -> Self {
        Encoder { writer }
    }

    /// Write one PGM image in either `raw` or `plain` format.
    ///
    /// No checks are made on the number of `plain` images
    /// written. The netpbm spec dictates that `plain` files
    /// should only have a single image. It is up to the client
    /// caller to ensure they invoke this method only once for
    /// `plain` files.
    ///
    pub fn write(
        &mut self,
        encoding: EncodingType,
        width: u32,
        height: u32,
        bit_depth: u16,
        samples: &[u8],
    ) -> Result<(), NetpbmError> {
        let info = Info::new_pgm(encoding, width, height, bit_depth)?;
        info.validate_u8_samples(samples)?;

        match encoding {
            EncodingType::Raw => self.write_raw_u8(&info, samples),
            EncodingType::Plain => self.write_plain_u8(&info, samples),
        }
    }

    /// Write one PGM image in either `raw` or `plain` format.
    ///
    /// If the bit depth is less than 256, samples will be
    /// truncated to the lower byte.
    ///
    /// No checks are made on the number of `plain` images
    /// written. The netpbm spec dictates that `plain` files
    /// should only have a single image. It is up to the client
    /// caller to ensure they invoke this method only once for
    /// `plain` files.
    ///
    pub fn write_wide(
        &mut self,
        encoding: EncodingType,
        width: u32,
        height: u32,
        bit_depth: u16,
        samples: &[u16],
    ) -> Result<(), NetpbmError> {
        let info = Info::new_pgm(encoding, width, height, bit_depth)?;
        info.validate_u16_samples(samples)?;

        match encoding {
            EncodingType::Raw => self.write_raw_u16(&info, samples),
            EncodingType::Plain => self.write_plain_u16(&info, samples),
        }
    }

    /// Write a PGM image with `raw` encoding.
    fn write_raw_u8(&mut self, info: &Info, samples: &[u8]) -> Result<(), NetpbmError> {
        let mut buf = Self::build_header(info);
        buf.extend(samples);

        self.writer.write_all(&buf)?;

        Ok(())
    }

    /// Write a PPM image with `plain` encoding.
    fn write_plain_u8(&mut self, info: &Info, samples: &[u8]) -> Result<(), NetpbmError> {
        let mut buf = Self::build_header(info).to_vec();
        buf.extend(Self::build_lines_u8(samples));

        self.writer.write_all(&buf)?;

        Ok(())
    }

    /// Write a PGM image with `raw` encoding.
    fn write_raw_u16(&mut self, info: &Info, samples: &[u16]) -> Result<(), NetpbmError> {
        let mut buf = Self::build_header(info);

        // Truncate samples to one byte if the bit depth is less than 256.
        if !info.bit_depth.is_multi_byte() {
            buf.extend(samples.iter().map(|s| (s & 0xFF) as u8));
        } else {
            // netpbm specifies that multi-byte samples are big-endian.
            buf.extend(samples.iter().flat_map(|s| s.to_be_bytes()));
        }

        self.writer.write_all(&buf)?;

        Ok(())
    }

    /// Write a PPM image with `plain` encoding.
    fn write_plain_u16(&mut self, info: &Info, samples: &[u16]) -> Result<(), NetpbmError> {
        let mut buf = Self::build_header(info).to_vec();
        buf.extend(Self::build_lines_u16(samples));

        self.writer.write_all(&buf)?;

        Ok(())
    }

    /// Build a PGM header.
    fn build_header(info: &Info) -> Vec<u8> {
        format!(
            "{}\n{} {} {}\n",
            info.format.magic(),
            info.width,
            info.height,
            info.bit_depth
        )
        .as_bytes()
        .to_vec()
    }

    /// Build the raster as lines of ASCII sample values.
    fn build_lines_u8(samples: &[u8]) -> Vec<u8> {
        samples
            .iter()
            .flat_map(|s| format!("{}\n", s).as_bytes().to_owned())
            .collect()
    }

    /// Build the raster as lines of ASCII sample values.
    fn build_lines_u16(samples: &[u16]) -> Vec<u8> {
        samples
            .iter()
            .flat_map(|s| format!("{}\n", s).as_bytes().to_owned())
            .collect()
    }
}

/// PGM decoder.
#[derive(Debug)]
pub struct Decoder<R: io::Read> {
    reader: R,
}

impl<R: io::Read> Decoder<R> {
    /// Create a new PGM decoder with the given reader.
    pub fn new(reader: R) -> Self {
        Decoder { reader }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct ImageBuffer {
        buffer: Vec<u8>,
    }

    impl ImageBuffer {
        fn new() -> Self {
            ImageBuffer { buffer: Vec::new() }
        }
    }

    impl io::Write for ImageBuffer {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_write_ppm_raw() {
        let mut enc = Encoder::new(ImageBuffer::new());

        let data: Vec<u8> = vec![
            255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        ];
        let expected = [
            80, 53, 10, 52, 32, 52, 32, 50, 53, 53, 10, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
            255, 0, 255, 0, 255, 0,
        ];

        let res = enc.write(EncodingType::Raw, 4, 4, 255, &data);
        assert!(res.is_ok());
        assert_eq!(enc.writer.buffer[..], expected[..]);
    }

    #[test]
    fn test_write_ppm_plain() {
        let mut enc = Encoder::new(ImageBuffer::new());

        let data: Vec<u8> = vec![
            255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0,
        ];
        let expected = format!(
            "P2\n4 4 255\n255\n0\n255\n0\n255\n0\n255\n0\n255\n0\n255\n0\n255\n0\n255\n0\n"
        );

        let res = enc.write(EncodingType::Plain, 4, 4, 255, &data);
        assert!(res.is_ok());
        assert_eq!(enc.writer.buffer[..], *expected.as_bytes());
    }

    #[test]
    fn test_write_ppm_wide_raw() {
        let mut enc = Encoder::new(ImageBuffer::new());

        let data: Vec<u16> = vec![
            1056, 0, 1056, 0, 1056, 0, 1056, 0, 1056, 0, 1056, 0, 1056, 0, 1056, 0,
        ];
        let expected = [
            80, 53, 10, 52, 32, 52, 32, 50, 48, 52, 56, 10, 4, 32, 0, 0, 4, 32, 0, 0, 4, 32, 0, 0,
            4, 32, 0, 0, 4, 32, 0, 0, 4, 32, 0, 0, 4, 32, 0, 0, 4, 32, 0, 0,
        ];

        let res = enc.write_wide(EncodingType::Raw, 4, 4, 2048, &data);
        assert!(res.is_ok());
        assert_eq!(enc.writer.buffer[..], expected[..]);
        assert!(true)
    }

    #[test]
    fn test_write_ppm_wide_plain() {
        let mut enc = Encoder::new(ImageBuffer::new());

        let data: Vec<u16> = vec![
            1056, 0, 1056, 0, 1056, 0, 1056, 0, 1056, 0, 1056, 0, 1056, 0, 1056, 0,
        ];
        let expected = format!(
            "P2\n4 4 2048\n1056\n0\n1056\n0\n1056\n0\n1056\n0\n1056\n0\n1056\n0\n1056\n0\n1056\n0\n"
        );

        let res = enc.write_wide(EncodingType::Plain, 4, 4, 2048, &data);
        assert!(res.is_ok());
        assert_eq!(enc.writer.buffer[..], *expected.as_bytes());
    }
}
