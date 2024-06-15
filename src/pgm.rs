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

pub struct PGMEncoder {
    //
}

pub struct PGMDecoder {
    //
}
