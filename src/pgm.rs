use crate::{fields::*, NetpbmError, NetpbmFileFormat};

/// PGM (Portable Gray Map) image.
///
/// Each PGM image fundamentally consists of the image width,
/// the image height, the bit depth, and a sequence of rows of
/// grey values. There are `height` number of rows, each with
/// `width` grey values.
///
/// Each PGM image also has associated with it a magic number,
/// which is either the bytes `P2` or `P5`. The magic number indicates
/// the PGM file format (see PgmFile for details). The file format
/// indicates how the PGM file is serialized.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PgmImage {
    grey_values: Vec<u16>,
    width: ImageDim,
    height: ImageDim,
    bit_depth: BitDepth,
}

/// netpbm represents the alpha channel with a PgmImage.
type OpacityMask = PgmImage;

impl PgmImage {
    /// Create a new PGM image from the grey values, image width,
    /// image height, and bit depth.
    pub fn from_grey_values(
        grey_values: Vec<u16>,
        width: u32,
        height: u32,
        bit_depth: u32,
    ) -> Result<PgmImage, NetpbmError> {
        let width = ImageDim::new(width)?;
        let height = ImageDim::new(height)?;

        // The length of the grey value buffer should be
        // equal to the image width times the image height.
        if grey_values.len() as u32 != width.value() * height.value() {
            return Err(NetpbmError::MalformedInitArray {
                length: grey_values.len() as u32,
                width,
                height,
            });
        }

        let bit_depth = BitDepth::new(bit_depth)?;

        // All grey values must be less than the given bit depth.
        for &grey in grey_values.iter() {
            if grey as u32 > bit_depth.value() {
                return Err(NetpbmError::OversizedSample {
                    sample: grey,
                    bit_depth,
                });
            }
        }

        Ok(PgmImage {
            grey_values,
            width,
            height,
            bit_depth,
        })
    }

    /// Get a ref to the grey value at position (x, y) in the image.
    pub fn get(&self, x: usize, y: usize) -> u16 {
        self.grey_values[y * self.width.value() as usize + x]
    }

    /// Iterate over the grey values.
    pub fn iter(&self) -> core::slice::Iter<'_, u16> {
        self.grey_values.iter()
    }
}

/// PGM `raw` file format.
///
/// `raw` PGM files consist of a sequence of PGM images.
/// Grey values are serialized as unsigned binary integers.
/// The `raw` format uses the magic number `P5`.
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PgmRaw {
    images: Vec<PgmImage>,
}

impl PgmRaw {
    /// Make an empty PGM `raw` file.
    pub fn new() -> Self {
        PgmRaw { images: Vec::new() }
    }

    /// Add a new image to the image list.
    pub fn add_image(&mut self, image: PgmImage) {
        self.images.push(image)
    }

    /// Iterate over the contained PGM images.
    pub fn iter(&self) -> core::slice::Iter<'_, PgmImage> {
        self.images.iter()
    }
}

impl From<Vec<PgmImage>> for PgmRaw {
    /// Make a PGM `raw` file given a list of PGM images.
    fn from(images: Vec<PgmImage>) -> Self {
        PgmRaw { images }
    }
}

impl Default for PgmRaw {
    fn default() -> Self {
        Self::new()
    }
}

/// PGM `plain` file format.
///
/// `plain` PGM files consist of a single PGM image.
/// Grey values are written as ASCII-encoded decimal numbers.
/// The `plain` format uses the magic number `P2`.
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PgmPlain {
    image: PgmImage,
}

impl PgmPlain {
    /// Make an empty PGM `plain` file.
    pub fn new(image: PgmImage) -> Self {
        PgmPlain { image }
    }

    /// Get a ref to the contained PGM image.
    fn image_ref(&self) -> &PgmImage {
        &self.image
    }
}

impl From<PgmImage> for PgmPlain {
    /// Make a PGM `plain` file given a single PGM image.
    fn from(image: PgmImage) -> Self {
        PgmPlain { image }
    }
}
