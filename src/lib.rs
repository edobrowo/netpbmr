use fields::*;
use std::error::Error;
use std::fmt;

pub mod fields;
pub mod ppm;

#[derive(Debug, Clone)]
pub enum NetpbmError {
    InvalidBitDepth {
        value: u32,
    },
    InvalidImageDim {
        value: u32,
    },
    MalformedInitArray {
        length: u32,
        width: ImageDim,
        height: ImageDim,
    },
    OversizedChannel {
        channel: u8,
        bitdepth: BitDepth,
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
                "Invalid bit depth: {} (should be in range [{}, {}]",
                value,
                BitDepth::BITDEPTH_MIN,
                BitDepth::BITDEPTH_MAX
            ),
            InvalidImageDim { ref value } => write!(
                f,
                "Invalid image dimension: {} (should be greater than 0)",
                value
            ),
            MalformedInitArray {
                length,
                width,
                height,
            } => write!(
                f,
                "Color array size {} does not match expected image size {} * {}",
                length, width, height
            ),
            OversizedChannel {
                channel,
                bitdepth: bit_depth,
            } => {
                write!(
                    f,
                    "Color {} is larger than bit depth {}",
                    channel, bit_depth
                )
            }
        }
    }
}
