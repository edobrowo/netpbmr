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

use crate::{formats::EncodingType, samples::*, Info};
use crate::{Image, NetpbmError, TypeInfo};
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
    ///
    /// If the bit depth is less than 256, samples will be
    /// truncated to the lower byte.
    ///
    pub fn write<T: SampleType>(
        &mut self,
        width: u32,
        height: u32,
        bit_depth: u16,
        channels: u32,
        type_info: &[TypeInfo],
        samples: &[T::Sample],
    ) -> Result<(), NetpbmError> {
        let info = Info::new_pam(width, height, bit_depth, channels)?;
        let image = Image::new::<T>(samples, info)?;

        let mut buf = Self::build_header(&image, type_info);

        match image.samples {
            SampleBuffer::EIGHT(samples) => {
                buf.extend(samples);
            }
            SampleBuffer::SIXTEEN(samples) => {
                // Truncate samples to one byte if the bit depth is less than 256.
                if !image.info.bit_depth.is_multi_byte() {
                    buf.extend(samples.iter().map(|s| (s & 0xFF) as u8));
                } else {
                    // netpbm specifies that multi-byte samples are big-endian.
                    buf.extend(samples.iter().flat_map(|s| s.to_be_bytes()));
                }
            }
        }

        self.writer.write_all(&buf)?;

        Ok(())
    }

    /// Build a PAM header.
    fn build_header(image: &Image, _: &[TypeInfo]) -> Vec<u8> {
        // TODO : typeinfo lol
        format!(
            "
            {}\n
            WIDTH {}\n
            HEIGHT {}\n
            DEPTH {}\n
            MAXVAL {}\n
            ENDHDR
            ",
            image.format().magic(),
            image.width(),
            image.height(),
            image.channels(),
            image.bit_depth(),
        )
        .as_bytes()
        .to_vec()
    }
}

/// PAM decoder.
#[derive(Debug)]
pub struct Decoder<R: io::Read> {
    reader: R,
}

impl<R: io::Read> Decoder<R> {
    /// Create a new PAM decoder with the given reader.
    pub fn new(reader: R) -> Self {
        Decoder { reader }
    }
}
