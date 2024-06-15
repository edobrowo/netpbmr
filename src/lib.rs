use formats::*;
use std::error::Error;
use std::fmt;

pub mod formats;
pub mod pam;
pub mod pbm;
pub mod pgm;
pub mod ppm;
pub mod samples;

use samples::{SampleBuffer, SampleType};

/// netpbm errors.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NetpbmError {
    /// The bit depth is out of the acceptable range.
    InvalidBitDepth { value: u16 },
    /// The image dim value is out of the acceptable range.
    InvalidImageDim { value: u32 },
    /// The channel depth is out of the acceptable range.
    InvalidChannelDepth { value: u32 },
    /// The data size does not match the image dimensions.
    MalformedInitArray {
        data_size: usize,
        width: ImageDim,
        height: ImageDim,
    },
    /// A sample value is greater than the provided bit depth.
    OversizedSample { offset: usize, bit_depth: BitDepth },
    /// The length of a tuple is greater than the provided channel depth.
    OversizedTuple {
        length: usize,
        channel_depth: ChannelDepth,
    },
}

impl Error for NetpbmError {
    fn description(&self) -> &str {
        "netpbm error"
    }
}

impl fmt::Display for NetpbmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::NetpbmError::*;
        match *self {
            InvalidBitDepth { ref value } => write!(
                f,
                "Bit depth {} should be in range [{}, {}]",
                value,
                BitDepth::MIN,
                BitDepth::MAX,
            ),
            InvalidImageDim { ref value } => {
                write!(f, "Image dimension {} should be greater than 0", value)
            }
            InvalidChannelDepth { ref value } => {
                write!(f, "Channel depth {} should be greater than 0", value)
            }
            MalformedInitArray {
                ref data_size,
                ref width,
                ref height,
            } => {
                write!(
                    f,
                    "Data size {} does not match image dimensions ({}, {})",
                    data_size, width, height
                )
            }
            OversizedSample {
                ref offset,
                ref bit_depth,
            } => {
                write!(
                    f,
                    "Sample value at byte [{}] is larger than the expected bit depth {}",
                    offset, bit_depth
                )
            }
            OversizedTuple {
                ref length,
                ref channel_depth,
            } => {
                write!(
                    f,
                    "Tuple size {} is larger than the expected channel depth {}",
                    length, channel_depth
                )
            }
        }
    }
}

/// Lightweight image wrapper that maintains a ref to a buffer
/// of samples. On creation, the provided header values are
/// bounds-checked and samples are validated against the
/// bit-depth and image dimensions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Image<'a> {
    info: Info,
    samples: SampleBuffer<'a>,
}

impl<'a> Image<'a> {
    /// Create a new image.
    pub fn new<T: SampleType>(samples: &'a [T::Sample], info: Info) -> Result<Self, NetpbmError> {
        T::validate_samples(&info, samples)?;
        let samples = T::to_sample_buffer(samples);
        Ok(Self { samples, info })
    }

    // Get the netpbm format.
    pub fn format(&self) -> NetpbmFormat {
        self.info.format.clone()
    }

    // Get the encoding type.
    pub fn encoding(&self) -> EncodingType {
        self.info.encoding.clone()
    }

    // Get the width as a u32.
    pub fn width(&self) -> u32 {
        self.info.width.value()
    }

    // Get the height as a u32.
    pub fn height(&self) -> u32 {
        self.info.height.value()
    }

    // Get the bit depth as a u16.
    pub fn bit_depth(&self) -> u16 {
        self.info.bit_depth.value()
    }

    // Get the number of channels as a u32.
    pub fn channels(&self) -> u32 {
        self.info.channels.value()
    }

    // Get a ref to the image data.
    pub fn samples(&self) -> &SampleBuffer {
        &self.samples
    }
}
