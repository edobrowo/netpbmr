use fields::*;
use std::error::Error;
use std::fmt;
use std::io;

pub mod fields;
pub mod pam;
pub mod pbm;
pub mod pgm;
pub mod pnm;
pub mod ppm;

pub trait NetpbmFileFormat {
    /// Write the netpbm file into a writer.
    fn write_to<W: io::Write>(&self, writer: &mut W) -> io::Result<usize>;

    /// Parse a netpbm file from a reader.
    fn parse<R: io::Read>(reader: &mut R) -> Self;
}

/// netpbm errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NetpbmError {
    /// The bit depth is out of the acceptable range.
    InvalidBitDepth { value: u32 },
    /// The image dim value is out of the acceptable range.
    InvalidImageDim { value: u32 },
    /// The channel depth is out of the acceptable range.
    InvalidChannelDepth { value: u32 },
    /// The size of the sample array is not equal to the
    /// product of the given width and height.
    MalformedInitArray {
        length: u32,
        width: ImageDim,
        height: ImageDim,
    },
    /// A sample value is greater than the provided bit depth.
    OversizedSample { sample: u16, bit_depth: BitDepth },
    /// The length of a tuple is greater than the provided channel depth.
    OversizedTuple {
        length: u32,
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
                BitDepth::MAX
            ),
            InvalidImageDim { ref value } => {
                write!(f, "Image dimension {} should be greater than 0", value)
            }
            InvalidChannelDepth { ref value } => {
                write!(f, "Channel depth {} should be greater than 0", value)
            }
            MalformedInitArray {
                ref length,
                ref width,
                ref height,
            } => write!(
                f,
                "Sample array size {} does not match expected image size {} * {} = {}",
                length,
                width,
                height,
                width.value() * height.value()
            ),
            OversizedSample {
                sample: ref channel,
                ref bit_depth,
            } => {
                write!(
                    f,
                    "Sample value {} is larger than the expected bit depth {}",
                    channel, bit_depth
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
