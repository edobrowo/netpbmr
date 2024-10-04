//! PBM (Portable Bit Map) image encoding and decoding.
//!
//! Each PBM image fundamentally consists of the image width,
//! the image height, and a sequence of bits.
//! There are `height` number of rows, each with `width` bits.
//!
//! PBM `raw` files consist of a sequence of PBM images.
//! Bits are byte-packed, with optional padding at the end of each
//! scan line. The `raw` format uses the magic number `P4`.
//!
//! PBM `plain` files consist of a single PBM image.
//! Bits are written as ASCII-encoded `0` or `1`.
//! The `plain` format uses the magic number `P1`.
//!

use std::io;

use crate::formats::decode;
use crate::{EncodingType, Info, MagicNumber, NetpbmError, NetpbmFormat};

/// PBM encoder.
#[derive(Debug)]
pub struct Encoder<W: io::Write> {
    writer: W,
}

impl<W: io::Write> Encoder<W> {
    /// Create a new PBM encoder with the given writer.
    pub fn new(writer: W) -> Self {
        Encoder { writer }
    }

    /// Write one PBM image in either `raw` or `plain` format.
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
        samples: &[u8],
    ) -> Result<(), NetpbmError> {
        let info = Info::new_pbm(encoding, width, height)?;
        info.validate_u8_samples(samples)?;
        match encoding {
            EncodingType::Raw => self.write_raw(&info, samples),
            EncodingType::Plain => self.write_plain(&info, samples),
        }
    }

    /// Write a PBM image with `raw` encoding.
    fn write_raw(&mut self, info: &Info, samples: &[u8]) -> Result<(), NetpbmError> {
        let mut buf = Self::build_header(info);

        // Pack bytes. Add right-side padding if there are remainder bits.
        let packed_bytes = samples
            .chunks(8)
            .map(|x| x.iter().enumerate().fold(0, |a, (i, b)| a | (b << (7 - i))));
        buf.extend(packed_bytes);

        self.writer.write_all(&buf)?;

        Ok(())
    }

    /// Write a PBM image with `plain` encoding.
    fn write_plain(&mut self, info: &Info, samples: &[u8]) -> Result<(), NetpbmError> {
        let mut buf = Self::build_header(info).to_vec();
        buf.extend(Self::build_lines(samples));

        self.writer.write_all(&buf)?;

        Ok(())
    }

    /// Build a PBM header.
    fn build_header(info: &Info) -> Vec<u8> {
        format!("{}\n{} {}\n", info.format.magic(), info.width, info.height,)
            .as_bytes()
            .to_vec()
    }

    /// Build the raster as lines of ASCII sample values.
    fn build_lines(samples: &[u8]) -> Vec<u8> {
        // Write 35 samples per line
        samples
            .chunks(35)
            .flat_map(|chunk| {
                let line = chunk
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                format!("{}\n", line).as_bytes().to_owned()
            })
            .collect()
    }
}

/// PBM decoder.
#[derive(Debug)]
pub struct Decoder<R: io::Read> {
    reader: R,
}

impl<R: io::Read> Decoder<R> {
    /// Create a new PBM decoder with the given reader.
    pub fn new(reader: R) -> Self {
        Decoder { reader }
    }

    /// Read from the provided buffer and fill the `Info` metadata struct.
    pub fn read(&mut self, buf: &mut [u8]) -> Result<Info, NetpbmError> {
        let mut img_buf = Vec::new();
        self.reader.read_to_end(&mut img_buf)?;

        let magic_number: [u8; 2] = img_buf[0..2].try_into()?;
        let format = match MagicNumber::from_bytes(&magic_number) {
            Some(magic_number) if magic_number == NetpbmFormat::PBMRaw.magic() => {
                NetpbmFormat::PBMRaw
            }
            Some(magic_number) if magic_number == NetpbmFormat::PBMPlain.magic() => {
                NetpbmFormat::PBMPlain
            }
            Some(magic_number) => {
                return Err(NetpbmError::IOOperationFailed {
                    info: format!("Invalid magic number: {}", magic_number),
                })
            }
            None => {
                return Err(NetpbmError::IOOperationFailed {
                    info: format!(
                        "Unexpected bytes at magic number position: {:?}",
                        magic_number
                    ),
                })
            }
        };

        match format {
            NetpbmFormat::PBMRaw => self.read_raw(&img_buf, buf),
            NetpbmFormat::PBMPlain => self.read_plain(&img_buf, buf),
            _ => unreachable!(),
        }
    }

    /// Read a PBM `raw` image to the provided buffer and fill the `Info` metadata struct.
    ///
    /// Assumes magic number bytes can be skipped.
    ///
    fn read_raw(&mut self, img_buf: &[u8], buf: &mut [u8]) -> Result<Info, NetpbmError> {
        Info::new_pbm(EncodingType::Raw, 1, 1)
    }

    /// Read a PBM `plain` image to the provided buffer and fill the `Info` metadata struct.
    ///
    /// Assumes magic number bytes can be skipped.
    ///
    fn read_plain(&mut self, img_buf: &[u8], buf: &mut [u8]) -> Result<Info, NetpbmError> {
        Info::new_pbm(EncodingType::Plain, 1, 1)
    }

    /// Reads the next field.
    ///
    /// Assumes the last read byte
    ///
    fn read_field(&mut self, buf: &mut [u8]) -> Result<(), NetpbmError> {
        Ok(())
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
    fn test_write_pbm_raw() {
        let mut enc = Encoder::new(ImageBuffer::new());

        let data: Vec<u8> = vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
        let expected = [80, 52, 10, 52, 32, 51, 10, 170, 160];

        let res = enc.write(EncodingType::Raw, 4, 3, &data);
        assert!(res.is_ok());
        assert_eq!(enc.writer.buffer[..], expected[..]);
    }

    #[test]
    fn test_write_pbm_plain() {
        let mut enc = Encoder::new(ImageBuffer::new());

        let data: Vec<u8> = vec![
            1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
            0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
        ];
        let expected = format!(
            "P1\n7 6\n1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1\n0 1 0 1 0 1 0\n"
        );

        let res = enc.write(EncodingType::Plain, 7, 6, &data);
        assert!(res.is_ok());
        assert_eq!(enc.writer.buffer[..], *expected.as_bytes());
    }
}
