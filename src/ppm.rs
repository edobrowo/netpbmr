use crate::{fields::*, NetpbmError, NetpbmFileFormat};

/// PPM (Portable Pixel Map) image.
///
/// Each PPM image fundamentally consists of the image width,
/// the image height, the bit depth, and a sequence of rows of
/// color channel data. Each pixel is represented by a triplet
/// of color channel data (red, green, blue). There are `height`
/// number of rows, each with `width` color triplets.
///
/// Each PPM image also has associated with it a magic number,
/// which is either the bytes `P6` or `P3`. The magic number indicates
/// the PPM file format (see PpmFile for details). The file format
/// indicates how the PPM file is serialized.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PpmImage {
    colors: Vec<[u16; 3]>,
    width: ImageDim,
    height: ImageDim,
    bit_depth: BitDepth,
}

impl PpmImage {
    /// Create a new PPM image from the color channel values,
    /// image width, image height, and bit depth.
    pub fn from_colors(
        colors: Vec<[u16; 3]>,
        width: u32,
        height: u32,
        bit_depth: u32,
    ) -> Result<PpmImage, NetpbmError> {
        let width = ImageDim::new(width)?;
        let height = ImageDim::new(height)?;

        // The length of the color channel buffer should be
        // equal to the image width times the image height.
        if colors.len() as u32 != width.value() * height.value() {
            return Err(NetpbmError::MalformedInitArray {
                length: colors.len() as u32,
                width,
                height,
            });
        }

        let bit_depth = BitDepth::new(bit_depth)?;

        // All color channel values must be less than the given bit depth.
        for color in colors.iter() {
            if let Some(&channel) = color.iter().find(|&chan| *chan as u32 > bit_depth.value()) {
                return Err(NetpbmError::OversizedSample {
                    sample: channel,
                    bit_depth,
                });
            }
        }

        Ok(PpmImage {
            colors,
            width,
            height,
            bit_depth,
        })
    }

    /// Get a ref to the color triplet at position (x, y)
    /// in the image.
    pub fn get(&self, x: usize, y: usize) -> &[u16; 3] {
        &self.colors[y * self.width.value() as usize + x]
    }

    /// Iterate over the color triplets.
    pub fn iter(&self) -> core::slice::Iter<'_, [u16; 3]> {
        self.colors.iter()
    }
}

/// PPM `raw` file format.
///
/// `raw` PPM files consist of a sequence of PPM images.
/// Color channel values are serialized as unsigned
/// binary integers.
/// The `raw` format uses the magic number `P6`.
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PpmRaw {
    images: Vec<PpmImage>,
}

impl PpmRaw {
    /// Make an empty PPM `raw` file.
    pub fn new() -> Self {
        PpmRaw { images: Vec::new() }
    }

    /// Add a new image to the image list.
    pub fn add_image(&mut self, image: PpmImage) {
        self.images.push(image)
    }

    /// Iterate over the contained PPM images.
    pub fn iter(&self) -> core::slice::Iter<'_, PpmImage> {
        self.images.iter()
    }
}

impl NetpbmFileFormat for PpmRaw {
    fn magic_number(&self) -> MagicNumber {
        MagicNumber::P6
    }
}

impl From<Vec<PpmImage>> for PpmRaw {
    /// Make a PPM `raw` file given a list of PPM images.
    fn from(images: Vec<PpmImage>) -> Self {
        PpmRaw { images }
    }
}

impl Default for PpmRaw {
    fn default() -> Self {
        Self::new()
    }
}

/// PPM `plain` file format.
///
/// `plain` PPM files consist of a single PPM image.
/// Color channel values are written as ASCII-encoded
/// decimal numbers.
/// The `plain` format uses the magic number `P3`.
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PpmPlain {
    image: PpmImage,
}

impl PpmPlain {
    /// Make an empty PPM `plain` file.
    pub fn new(image: PpmImage) -> Self {
        PpmPlain { image }
    }

    /// Get a ref to the contained PPM image.
    fn image_ref(&self) -> &PpmImage {
        &self.image
    }
}

impl NetpbmFileFormat for PpmPlain {
    fn magic_number(&self) -> MagicNumber {
        MagicNumber::P3
    }
}

impl From<PpmImage> for PpmPlain {
    /// Make a PPM `plain` file given a single PPM image.
    fn from(image: PpmImage) -> Self {
        PpmPlain { image }
    }
}
