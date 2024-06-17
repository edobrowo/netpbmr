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
use crate::{Image, NetpbmError};
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
