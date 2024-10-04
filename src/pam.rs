//! PAM (Portable Arbitrary Map) image encoding and decoding.
//!
//! PAM generalizes the other netpbm formats. PAM images consist
//! of a 2D array of tuples. Tuple components are called samples.
//! All tuples in the same image must have the same length.
//! This is referred to as the channel depth, or depth for short.
//! The image depth dictates the number of channels in the image.
//!
//! The maxval of an image specifies the maximum value that a
//! sample can take. It is the generalized term for bit depth.
//!
//! Each PAM image fundamentally consists of the image width,
//! the image height, the depth, the maxval, and a sequence of rows
//! of tuples. Each tuple consists of `depth` number of samples.
//! There are `height` number of rows, each with `width` tuples,
//! with all tuple samples ordered left-to-right on each row.
//!
//! PAM images support an optional `tuple type` field. It is an ASCII
//! string providing semantic information about the data contained
//! in the PAM image.
//!
//! PAM files consist of a sequence of PAM images.
//! PAM files only have one format, which is serialized
//! similar what is done with the `raw` format of
//! PBM, PGM, and PPM. The PAM format uses the magic number `P7`.

use crate::Info;
use crate::{NetpbmError, TypeInfo};
use std::io;

/// PAM encoder.
#[derive(Debug)]
pub struct Encoder<W: io::Write> {
    writer: W,
}

impl<W: io::Write> Encoder<W> {
    /// Create a new PAM encoder with the given writer.
    pub fn new(writer: W) -> Self {
        Encoder { writer }
    }

    /// Write one PAM image.
    pub fn write(
        &mut self,
        width: u32,
        height: u32,
        bit_depth: u8,
        channels: u32,
        type_info: &TypeInfo,
        samples: &[u8],
    ) -> Result<(), NetpbmError> {
        let info = Info::new_pam(width, height, bit_depth as u16, channels)?;
        info.validate_u8_samples(samples)?;

        let mut buf = Self::build_header(&info, type_info);
        buf.extend(samples);

        self.writer.write_all(&buf)?;

        Ok(())
    }

    /// Write one PAM image.
    ///
    /// If the bit depth is less than 256, samples will be
    /// truncated to the lower byte.
    pub fn write_wide(
        &mut self,
        width: u32,
        height: u32,
        bit_depth: u16,
        channels: u32,
        type_info: &TypeInfo,
        samples: &[u16],
    ) -> Result<(), NetpbmError> {
        let info = Info::new_pam(width, height, bit_depth, channels)?;
        info.validate_u16_samples(samples)?;

        let mut buf = Self::build_header(&info, type_info);

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

    /// Build a PAM header.
    fn build_header(info: &Info, type_info: &TypeInfo) -> Vec<u8> {
        let mut header = format!(
            "{}\nWIDTH {}\nHEIGHT {}\nDEPTH {}\nMAXVAL {}\n",
            info.format.magic(),
            info.width,
            info.height,
            info.channels,
            info.bit_depth,
        );

        if let TypeInfo::Info(type_info) = type_info {
            for info in type_info {
                header += &format!("TUPLTYPE {}\n", info);
            }
        }

        header += "ENDHDR\n";

        header.into_bytes()
    }
}

// /// PAM decoder.
// #[derive(Debug)]
// pub struct Decoder<R: io::Read> {
//     reader: R,
// }

// impl<R: io::Read> Decoder<R> {
//     /// Create a new PAM decoder with the given reader.
//     pub fn new(reader: R) -> Self {
//         Decoder { reader }
//     }
// }

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
    fn test_write_pam_raw() {
        let mut enc = Encoder::new(ImageBuffer::new());

        let data: Vec<u8> = vec![
            127, 255, 0, 0, 127, 0, 255, 0, 127, 0, 0, 255, 127, 255, 255, 0, 127, 255, 255, 255,
            127, 0, 0, 0,
        ];
        let mut expected = "P7\nWIDTH 3\nHEIGHT 2\nDEPTH 4\nMAXVAL 255\nENDHDR\n"
            .to_string()
            .as_bytes()
            .to_vec();
        expected.extend([
            127, 255, 0, 0, 127, 0, 255, 0, 127, 0, 0, 255, 127, 255, 255, 0, 127, 255, 255, 255,
            127, 0, 0, 0,
        ]);

        let res = enc.write(3, 2, 255, 4, &TypeInfo::Empty, &data);
        assert!(res.is_ok());
        assert_eq!(enc.writer.buffer[..], expected[..]);
    }

    #[test]
    fn test_write_pam_wide() {
        let mut enc = Encoder::new(ImageBuffer::new());

        let data: Vec<u16> = vec![
            1056, 0, 0, 10, 0, 1056, 0, 10, 0, 0, 1056, 10, 1056, 1056, 0, 10, 1056, 1056, 1056,
            10, 0, 0, 0, 10,
        ];
        let mut expected = "P7\nWIDTH 3\nHEIGHT 2\nDEPTH 4\nMAXVAL 2048\nTUPLTYPE RGBA64\nENDHDR\n"
            .to_string()
            .as_bytes()
            .to_vec();
        expected.extend([
            4, 32, 0, 0, 0, 0, 0, 10, 0, 0, 4, 32, 0, 0, 0, 10, 0, 0, 0, 0, 4, 32, 0, 10, 4, 32, 4,
            32, 0, 0, 0, 10, 4, 32, 4, 32, 4, 32, 0, 10, 0, 0, 0, 0, 0, 0, 0, 10,
        ]);

        let type_info = TypeInfo::Info(vec!["RGBA64".to_string()]);
        let res = enc.write_wide(3, 2, 2048, 4, &type_info, &data);
        assert!(res.is_ok());
        assert_eq!(enc.writer.buffer[..], expected[..]);
    }
}
