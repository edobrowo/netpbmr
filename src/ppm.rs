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
