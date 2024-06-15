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

use crate::{Image, NetpbmError};

pub struct PBMEncoder {
    //
}

pub struct PBMDecoder {
    //
}
