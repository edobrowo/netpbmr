//! PPM (Portable Pixel Map) image format encoding and decoding.
//!
//! Each PPM image fundamentally consists of the image width,
//! the image height, the bit depth, and a sequence of rows of
//! color channel data. Each pixel is represented by a triplet
//! of color channel data (red, green, blue). There are `height`
//! number of rows, each with `width` color triplets.
//!
//! PPM `raw` files consist of a sequence of PPM images.
//! Color channel values are serialized as unsigned binary integers.
//! The `raw` format uses the magic number `P6`.
//!
//! PPM `plain` files consist of a single PPM image.
//! Color channel values are written as ASCII-encoded decimal numbers.
//! The `plain` format uses the magic number `P3`.
//!

use crate::{formats::EncodingType, samples::*, Info};
use crate::{Image, NetpbmError};
use std::io;

/// PPM encoder.
#[derive(Debug)]
pub struct Encoder<W: io::Write> {
    writer: W,
}

impl<W: io::Write> Encoder<W> {
    /// Create a new PPM encoder with the given writer.
    pub fn new(writer: W) -> Self {
        Encoder { writer }
    }

    /// Write one PPM image in either `raw` or `plain` format.
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
    pub fn write<T: SampleType>(
        &mut self,
        encoding: EncodingType,
        width: u32,
        height: u32,
        bit_depth: u16,
        samples: &[T::Sample],
    ) -> Result<(), NetpbmError> {
        let info = Info::new_ppm(encoding, width, height, bit_depth)?;
        let image = Image::new::<T>(samples, info)?;
        match encoding {
            EncodingType::Raw => self.write_raw::<T>(&image),
            EncodingType::Plain => self.write_plain::<T>(&image),
        }
    }

    /// Write a PPM image with `raw` encoding.
    fn write_raw<T: SampleType>(&mut self, image: &Image) -> Result<(), NetpbmError> {
        let mut buf = Self::build_header(image);

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

    /// Write a PPM image with `plain` encoding.
    fn write_plain<T: SampleType>(&mut self, image: &Image) -> Result<(), NetpbmError> {
        let mut buf = Self::build_header(image).to_vec();

        match image.samples {
            SampleBuffer::EIGHT(samples) => buf.extend(Self::build_triplets::<u8>(samples)),
            SampleBuffer::SIXTEEN(samples) => buf.extend(Self::build_triplets::<u16>(samples)),
        }

        self.writer.write_all(&buf)?;

        Ok(())
    }

    /// Build a PPM header.
    fn build_header(image: &Image) -> Vec<u8> {
        format!(
            "{}\n{} {} {}\n",
            image.format().magic(),
            image.width(),
            image.height(),
            image.bit_depth()
        )
        .as_bytes()
        .to_vec()
    }

    /// Build the raster as lines of ASCII RGB triplets.
    fn build_triplets<T: SampleType>(samples: &[T::Sample]) -> Vec<u8> {
        samples
            .chunks_exact(3)
            .flat_map(|triplet| {
                format!("{} {} {}\n", triplet[0], triplet[1], triplet[2])
                    .as_bytes()
                    .to_owned()
            })
            .collect()
    }
}

/// PPM decoder.
#[derive(Debug)]
pub struct Decoder<R: io::Read> {
    reader: R,
}

impl<R: io::Read> Decoder<R> {
    /// Create a new PPM decoder with the given reader.
    pub fn new(reader: R) -> Self {
        Decoder { reader }
    }
}
