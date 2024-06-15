use crate::{Info, NetpbmError, SampleBuffer, SampleType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PpmImage<'a> {
    info: Info,
    samples: SampleBuffer<'a>,
}

impl<'a> PpmImage<'a> {
    /// Create a new PPM image.
    pub fn new<T: SampleType>(
        samples: &[T::Sample],
        width: u32,
        height: u32,
        bit_depth: u16,
    ) -> Result<PpmImage, NetpbmError> {
        let info = Info::new_ppm(width, height, bit_depth)?;
        T::validate_samples(&info, samples)?;
        let samples = T::to_sample_buffer(samples);
        Ok(PpmImage { samples, info })
    }

    /// Get a ref to the image info.
    pub fn info(&self) -> &Info {
        &self.info
    }
}
