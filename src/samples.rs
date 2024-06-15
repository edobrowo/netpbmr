use crate::{Info, NetpbmError};

/// Indicates how to encode a PNM file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EncodingType {
    /// Sample data is serialized as bytes.
    Raw,

    /// Sample data is written as ASCII integers separated
    /// by whitespace.
    Plain,
}

/// Generalizes over u8 and u16 since netpbm permits
/// samples to be either 8- or 16-bit.
pub trait SampleType {
    /// The sample type, either u8 or u16.
    type Sample;

    /// Validate that the samples agree with the header info.
    fn validate_samples(info: &Info, samples: &[Self::Sample]) -> Result<(), NetpbmError>;

    /// Convert the Sample slice into a SampleBuffer.
    fn to_sample_buffer(samples: &[Self::Sample]) -> SampleBuffer;
}

impl SampleType for u8 {
    type Sample = u8;

    fn validate_samples(info: &Info, samples: &[Self::Sample]) -> Result<(), NetpbmError> {
        info.validate_u8_samples(samples)
    }

    fn to_sample_buffer(samples: &[Self::Sample]) -> SampleBuffer {
        SampleBuffer::EIGHT(samples)
    }
}

impl SampleType for u16 {
    type Sample = u16;

    fn validate_samples(info: &Info, samples: &[Self::Sample]) -> Result<(), NetpbmError> {
        info.validate_u16_samples(samples)
    }

    fn to_sample_buffer(samples: &[Self::Sample]) -> SampleBuffer {
        SampleBuffer::SIXTEEN(samples)
    }
}

/// Convenience sample buffer.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SampleBuffer<'a> {
    EIGHT(&'a [u8]),
    SIXTEEN(&'a [u16]),
}