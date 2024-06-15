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

use crate::{Image, NetpbmError};
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
