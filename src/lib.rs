use fields::*;
use std::error::Error;
use std::fmt;

pub mod fields;
pub mod pbm;
pub mod pgm;
pub mod pnm;
pub mod ppm;

/// A netpbm file format must supply its associated magic number.
pub trait NetpbmFileFormat {
    fn magic_number(&self) -> MagicNumber;
}

/// General netpbm-related error enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NetpbmError {
    /// The provided bit depth value is out of the acceptable range.
    InvalidBitDepth { value: u32 },
    /// The provided image dim value is out of the acceptable range.
    InvalidImageDim { value: u32 },
    /// The initialization color value array has a size unequal to
    /// the provide width times provided height.
    MalformedInitArray {
        length: u32,
        width: ImageDim,
        height: ImageDim,
    },
    /// The color channel value is larger than the provided bit depth.
    OversizedChannel { channel: u16, bit_depth: BitDepth },
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
                BitDepth::BIT_DEPTH_MIN,
                BitDepth::BIT_DEPTH_MAX
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
            OversizedChannel { channel, bit_depth } => {
                write!(
                    f,
                    "Color {} is larger than bit depth {}",
                    channel, bit_depth
                )
            }
        }
    }
}
