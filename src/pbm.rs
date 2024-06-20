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

use crate::NetpbmError;
use crate::{formats::EncodingType, Info};
use std::io;

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
    pub fn write<T: SampleType>(
        &mut self,
        encoding: EncodingType,
        width: u32,
        height: u32,
        samples: &[T::Sample],
    ) -> Result<(), NetpbmError> {
        let info = Info::new_pbm(encoding, width, height)?;
        let image = Image::new::<T>(samples, info)?;
        match encoding {
            EncodingType::Raw => self.write_raw::<T>(&image),
            EncodingType::Plain => self.write_plain::<T>(&image),
        }
    }

    /// Write a PBM image with `raw` encoding.
    fn write_raw<T: SampleType>(&mut self, image: &Image) -> Result<(), NetpbmError> {
        let mut buf = Self::build_header(image);

        match image.samples {
            SampleBuffer::EIGHT(samples) => {
                // TODO : packing
            }
            SampleBuffer::SIXTEEN(samples) => {
                // TODO : packing
            }
        }

        self.writer.write_all(&buf)?;

        Ok(())
    }

    /// Write a PBM image with `plain` encoding.
    fn write_plain<T: SampleType>(&mut self, image: &Image) -> Result<(), NetpbmError> {
        let mut buf = Self::build_header(image).to_vec();

        match image.samples {
            SampleBuffer::EIGHT(samples) => buf.extend(Self::build_lines::<u8>(samples)),
            SampleBuffer::SIXTEEN(samples) => buf.extend(Self::build_lines::<u16>(samples)),
        }

        self.writer.write_all(&buf)?;

        Ok(())
    }

    /// Build a PBM header.
    fn build_header(image: &Image) -> Vec<u8> {
        format!(
            "{}\n{} {}\n",
            image.format().magic(),
            image.width(),
            image.height(),
        )
        .as_bytes()
        .to_vec()
    }

    /// Build the raster as lines of ASCII sample values.
    fn build_lines<T: SampleType>(samples: &[T::Sample]) -> Vec<u8> {
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
}
