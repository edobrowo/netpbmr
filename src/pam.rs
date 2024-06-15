use crate::{fields::*, NetpbmError};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PamImage {
    tuples: Vec<Vec<u16>>,
    width: ImageDim,
    height: ImageDim,
    max_val: BitDepth,
    channel_depth: ChannelDepth,
    type_info: TypeInfo,
}

impl PamImage {
    /// Create a new PAM image from the sample values,
    /// image width, image height, bit depth, channel depth,
    /// and type info.
    pub fn from_samples(
        tuples: Vec<Vec<u16>>,
        width: u32,
        height: u32,
        bit_depth: u32,
        channel_depth: u32,
        type_info: TypeInfo,
    ) -> Result<PamImage, NetpbmError> {
        let width = ImageDim::new(width)?;
        let height = ImageDim::new(height)?;

        // The length of the tuple buffer should be
        // equal to the image width times the image height.
        if tuples.len() as u32 != width.value() * height.value() {
            return Err(NetpbmError::MalformedInitArray {
                length: tuples.len() as u32,
                width,
                height,
            });
        }

        let channel_depth = ChannelDepth::new(channel_depth)?;

        // All tuple sizes must equal the channel depth.
        for tuple in tuples.iter() {
            if tuple.len() as u32 != channel_depth.value() {
                return Err(NetpbmError::OversizedTuple {
                    length: tuple.len() as u32,
                    channel_depth,
                });
            }
        }

        let bit_depth = BitDepth::new(bit_depth)?;

        // All sample values must be less than the given bit depth.
        for tuple in tuples.iter() {
            if let Some(&sample) = tuple.iter().find(|&chan| *chan as u32 > bit_depth.value()) {
                return Err(NetpbmError::OversizedSample { sample, bit_depth });
            }
        }

        Ok(PamImage {
            tuples,
            width,
            height,
            max_val: bit_depth,
            channel_depth,
            type_info,
        })
    }

    /// Get a ref to the tuple at position (x, y) in the image.
    pub fn get(&self, x: usize, y: usize) -> &Vec<u16> {
        &self.tuples[y * self.width.value() as usize + x]
    }

    /// Iterate over the tuples.
    pub fn iter(&self) -> core::slice::Iter<'_, Vec<u16>> {
        self.tuples.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PamRaw {
    images: Vec<PamImage>,
}

impl PamRaw {
    /// Make an empty PAM file.
    pub fn new() -> Self {
        PamRaw { images: Vec::new() }
    }

    /// Add a new image to the image list.
    pub fn add_image(&mut self, image: PamImage) {
        self.images.push(image)
    }

    /// Iterate over the contained PAM images.
    pub fn iter(&self) -> core::slice::Iter<'_, PamImage> {
        self.images.iter()
    }
}

impl From<Vec<PamImage>> for PamRaw {
    /// Make a PAM file given a list of PAM images.
    fn from(images: Vec<PamImage>) -> Self {
        PamRaw { images }
    }
}

impl Default for PamRaw {
    fn default() -> Self {
        Self::new()
    }
}
