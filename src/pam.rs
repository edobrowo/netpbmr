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

use crate::{Image, NetpbmError};

pub struct PAMEncoder {
    //
}

pub struct PAMDecoder {
    //
}
