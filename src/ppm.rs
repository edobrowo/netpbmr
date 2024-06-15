use crate::{Info, NetpbmError, SampleBuffer, SampleType};

/// Lightweight PPM image that has a ref to a buffer of samples.
/// On creation, the provided header values are bounds-checked
/// and samples are validated against the bit-depth and image
/// dimensions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PPMImage<'a> {
    info: Info,
    samples: SampleBuffer<'a>,
}

impl<'a> PPMImage<'a> {
    /// Create a new PPM image with the provided header parameters
    /// and samples.
    pub fn new<T: SampleType>(
        samples: &[T::Sample],
        width: u32,
        height: u32,
        bit_depth: u16,
    ) -> Result<PPMImage, NetpbmError> {
        let info = Info::new_ppm(width, height, bit_depth)?;
        T::validate_samples(&info, samples)?;
        let samples = T::to_sample_buffer(samples);
        Ok(PPMImage { samples, info })
    }

    /// Get a ref to the image info.
    pub fn info(&self) -> &Info {
        &self.info
    }
}
