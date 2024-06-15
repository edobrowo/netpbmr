use crate::header::MagicNumber;

/// netpbm supports 4 types of images: PBM, PGM, PPM, and PAM.
/// PBM, PGM, and PPM are further divided into their `raw` and
/// `plain` variants.
///
/// There is an additional cateogry, PNM, which refers to
/// any of PBM, PGM, or PPM.
///
/// netpbm files consist of a sequence of netpbm images.
/// Each image has a header, sample data, whitespace, and
/// optional comments in the header and before the sample
/// data.
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NetpbmFormat {
    /// PBM (Portable Bit Map) image.
    ///
    /// Each PBM image fundamentally consists of the image width,
    /// the image height, and a sequence of bits.
    /// There are `height` number of rows, each with `width` bits.
    ///
    /// PBM `raw` files consist of a sequence of PBM images.
    /// Bits are byte-packed, with optional padding at the end of each
    /// scan line. The `raw` format uses the magic number `P4`.
    ///
    /// PBM `plain` files consist of a single PBM image.
    /// Bits are written as ASCII-encoded `0` or `1`.
    /// The `plain` format uses the magic number `P1`.
    ///
    PBMRaw,
    PBMPlain,

    /// PGM (Portable Gray Map) image.
    ///
    /// Each PGM image fundamentally consists of the image width,
    /// the image height, the bit depth, and a sequence of rows of
    /// grey values. There are `height` number of rows, each with
    /// `width` grey values.
    ///
    /// PGM `raw` files consist of a sequence of PGM images.
    /// Grey values are serialized as unsigned binary integers.
    /// The `raw` format uses the magic number `P5`.
    ///
    /// PGM `plain` files consist of a single PGM image.
    /// Grey values are written as ASCII-encoded decimal numbers.
    /// The `plain` format uses the magic number `P2`.
    PGMRaw,
    PGMPlain,

    /// PPM (Portable Pixel Map) image formats.
    ///
    /// Each PPM image fundamentally consists of the image width,
    /// the image height, the bit depth, and a sequence of rows of
    /// color channel data. Each pixel is represented by a triplet
    /// of color channel data (red, green, blue). There are `height`
    /// number of rows, each with `width` color triplets.
    ///
    /// PPM `raw` files consist of a sequence of PPM images.
    /// Color channel values are serialized as unsigned binary integers.
    /// The `raw` format uses the magic number `P6`.
    ///
    /// PPM `plain` files consist of a single PPM image.
    /// Color channel values are written as ASCII-encoded decimal numbers.
    /// The `plain` format uses the magic number `P3`.
    ///
    PPMRaw,
    PPMPlain,

    /// PAM (Portable Arbitrary Map) image format.
    ///
    /// PAM generalizes the other netpbm formats. PAM images consist
    /// of a 2D array of tuples. Tuple components are called samples.
    /// All tuples in the same image must have the same length.
    /// This is referred to as the channel depth, or depth for short.
    /// The image depth dictates the number of channels in the image.
    ///
    /// The maxval of an image specifies the maximum value that a
    /// sample can take. It is the generalized term for bit depth.
    ///
    /// Each PAM image fundamentally consists of the image width,
    /// the image height, the depth, the maxval, and a sequence of rows
    /// of tuples. Each tuple consists of `depth` number of samples.
    /// There are `height` number of rows, each with `width` tuples,
    /// with all tuple samples ordered left-to-right on each row.
    ///
    /// PAM images support an optional `tuple type` field. It is an ASCII
    /// string providing semantic information about the data contained
    /// in the PAM image.
    ///
    /// PAM files consist of a sequence of PAM images.
    /// PAM files only have one format, which is serialized
    /// similar what is done with the `raw` format of
    /// PBM, PGM, and PPM. The PAM format uses the magic number `P7`.
    ///
    PAM,
}

impl NetpbmFormat {
    /// PNM (Portable Any Map) generalizes PBM, PGM, and PPM formats.
    pub fn is_pnm(&self) -> bool {
        use NetpbmFormat::*;
        return matches!(
            self,
            PBMRaw | PBMPlain | PGMRaw | PGMPlain | PPMRaw | PPMPlain
        );
    }

    /// Get the magic number associated with the format.
    pub fn magic(&self) -> MagicNumber {
        use MagicNumber::*;
        use NetpbmFormat::*;
        match self {
            PBMPlain => P1,
            PGMPlain => P2,
            PPMPlain => P3,
            PBMRaw => P4,
            PGMRaw => P5,
            PPMRaw => P6,
            PAM => P7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic() {
        use MagicNumber::*;
        use NetpbmFormat::*;
        assert_eq!(PBMRaw.magic(), P4);
        assert_eq!(PBMPlain.magic(), P1);
        assert_eq!(PGMRaw.magic(), P5);
        assert_eq!(PGMPlain.magic(), P2);
        assert_eq!(PPMRaw.magic(), P6);
        assert_eq!(PPMPlain.magic(), P3);
        assert_eq!(PAM.magic(), P7);
    }

    #[test]
    fn test_pnm() {
        use NetpbmFormat::*;
        assert!(!PBMRaw.is_pnm());
        assert!(!PBMPlain.is_pnm());
        assert!(!PGMRaw.is_pnm());
        assert!(!PGMPlain.is_pnm());
        assert!(!PPMRaw.is_pnm());
        assert!(!PPMPlain.is_pnm());
        assert!(PAM.is_pnm());
    }
}
