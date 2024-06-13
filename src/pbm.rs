use crate::{fields::*, NetpbmError, NetpbmFileFormat};

/// PBM (Portable Bit Map) image.
///
/// Each PBM image fundamentally consists of the image width,
/// the image height, and a sequence of bits.
/// There are `height` number of rows, each with `width` bits.
///
/// Each PBM image also has associated with it a magic number,
/// which is either the bytes `P1` or `P4`. The magic number indicates
/// the PBM file format (see PbmFile for details). The file format
/// indicates how the PBM file is serialized.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PbmImage {
    bits: Vec<u8>,
    width: ImageDim,
    height: ImageDim,
}

impl PbmImage {
    /// Create a new PBM image from bits, image width, and image height.
    /// Assumes bits are byte-packed.
    pub fn from_bits(bits: Vec<u8>, width: u32, height: u32) -> Result<PbmImage, NetpbmError> {
        let width = ImageDim::new(width)?;
        let height = ImageDim::new(height)?;

        // The length of the bit buffer should be equal to
        // the image width times the image height divided by 8,
        // accounting for padding.
        let expected_size = (width.value() + 7) / 8 * height.value();
        if bits.len() as u32 != expected_size {
            return Err(NetpbmError::MalformedInitArray {
                length: bits.len() as u32,
                width,
                height,
            });
        }

        Ok(PbmImage {
            bits,
            width,
            height,
        })
    }

    /// Get a ref to the bit at position (x, y) in the image.
    pub fn get(&self, x: usize, y: usize) -> u8 {
        let index = y * self.width.value() as usize + x;
        let byte_index = index / 8;
        let byte = self.bits[byte_index];
        (byte & (1 << (index % 8))) >> (index % 8)
    }

    /// Iterate over the byte-packed bits.
    pub fn iter(&self) -> core::slice::Iter<'_, u8> {
        self.bits.iter()
    }
}

/// PBM `raw` file format.
///
/// `raw` PBM files consist of a sequence of PBM images.
/// Bits are serialized directly.
/// The `raw` format uses the magic number `P4`.
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PbmRaw {
    images: Vec<PbmImage>,
}

impl PbmRaw {
    /// Make an empty PBM `raw` file.
    pub fn new() -> Self {
        PbmRaw { images: Vec::new() }
    }

    /// Add a new image to the image list.
    pub fn add_image(&mut self, image: PbmImage) {
        self.images.push(image)
    }

    /// Iterate over the contained PBM images.
    pub fn iter(&self) -> core::slice::Iter<'_, PbmImage> {
        self.images.iter()
    }
}

impl From<Vec<PbmImage>> for PbmRaw {
    /// Make a PBM `raw` file given a list of PBM images.
    fn from(images: Vec<PbmImage>) -> Self {
        PbmRaw { images }
    }
}

impl Default for PbmRaw {
    fn default() -> Self {
        Self::new()
    }
}

/// PBM `plain` file format.
///
/// `plain` PBM files consist of a single PBM image.
/// Bits are written as ASCII-encoded `0` or `1`.
/// The `plain` format uses the magic number `P1`.
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PbmPlain {
    image: PbmImage,
}

impl PbmPlain {
    /// Make an empty PBM `plain` file.
    pub fn new(image: PbmImage) -> Self {
        PbmPlain { image }
    }

    /// Get a ref to the contained PBM image.
    fn image_ref(&self) -> &PbmImage {
        &self.image
    }
}

impl From<PbmImage> for PbmPlain {
    /// Make a PBM `plain` file given a single PBM image.
    fn from(image: PbmImage) -> Self {
        PbmPlain { image }
    }
}
