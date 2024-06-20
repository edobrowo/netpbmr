use formats::*;
use std::error::Error;
use std::fmt;
use std::io;

pub mod formats;
pub mod pam;
pub mod pbm;
pub mod pgm;
pub mod ppm;

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
    /// Encoding or decoding operation failed.
    IOOperationFailed { info: String },
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
            IOOperationFailed { ref info } => {
                write!(f, "IO operation failed: {}", info)
            }
        }
    }
}

impl From<io::Error> for NetpbmError {
    fn from(err: io::Error) -> NetpbmError {
        NetpbmError::IOOperationFailed {
            info: err.to_string(),
        }
    }
}
