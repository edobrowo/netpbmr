use crate::NetpbmError;
use std::fmt;

/// Magic number field.
///
/// Each netpbm format has an assigned magic number. Every
/// netpbm magic number consists of the two bytes
/// `PN`, where N is a natural number represented in ASCII.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MagicNumber {
    /// PBM Plain
    P1,
    /// PGM Plain
    P2,
    /// PPM Plain
    P3,
    /// PBM Raw
    P4,
    /// PGM Raw
    P5,
    /// PPM Raw
    P6,
    /// PAM
    P7,
}

impl fmt::Display for MagicNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let magic = match self {
            Self::P1 => "P1",
            Self::P2 => "P2",
            Self::P3 => "P3",
            Self::P4 => "P4",
            Self::P5 => "P5",
            Self::P6 => "P6",
            Self::P7 => "P7",
        };
        write!(f, "{}", magic)
    }
}

/// Bit depth field.
///
/// netpbm specifies that formats must specify the
/// maximum sample value in an image.
///
/// While PAM refers to this value as `maxval`, its type will
/// be referred to as `BitDepth` in general.
///
/// The bit depth must be between 1 and 65535 inclusive.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BitDepth(u16);

impl BitDepth {
    pub const MIN: u16 = 1;
    pub const MAX: u16 = u16::MAX;

    /// Create a new BitDepth from a u16.
    pub fn new(value: u16) -> Result<Self, NetpbmError> {
        if value > 0 {
            Ok(Self(value))
        } else {
            Err(NetpbmError::InvalidBitDepth { value })
        }
    }

    /// Get the BitDepth as a u16.
    pub fn value(&self) -> u16 {
        self.0
    }
}

impl fmt::Display for BitDepth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Image dimension field.
///
/// Image dimensions in netpbm must be positive integers. There is
/// no indication of maximum value, so the 32-bit unsigned
/// bound is used.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImageDim(u32);

impl ImageDim {
    // Create a new ImageDim from a u32.
    pub fn new(value: u32) -> Result<Self, NetpbmError> {
        if value > 0 {
            Ok(Self(value))
        } else {
            Err(NetpbmError::InvalidImageDim { value })
        }
    }

    /// Get the ImageDim as a u32.
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for ImageDim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Channel depth field.
///
/// The number of channels in a PAM image must be positive.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChannelDepth(u32);

impl ChannelDepth {
    // Create a new ChannelDepth from a u32.
    pub fn new(value: u32) -> Result<Self, NetpbmError> {
        if value > 0 {
            Ok(Self(value))
        } else {
            Err(NetpbmError::InvalidChannelDepth { value })
        }
    }

    /// Get the ChannelDepth as a u32.
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for ChannelDepth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Type info field.
///
/// Type info provides semantic information about the
/// data contained in a PAM image.
///
/// Type info is optional.
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeInfo {
    Info(String),
    Empty,
}

/// Metadata used during encoding.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Info {
    /// The image width.
    pub width: ImageDim,

    /// The image height.
    pub height: ImageDim,

    /// The image bit depth. This is the maximum value
    /// a sample can hold.
    pub bit_depth: BitDepth,

    /// The number of channels in the image. This is the
    /// number of samples per tuple. E.g., RGB has 3
    /// channels and greyscale has 1.
    pub channels: ChannelDepth,
}

type InfoRes = Result<Info, NetpbmError>;

impl Info {
    /// Create a new info struct for PBM images.
    pub fn new_pbm(width: u32, height: u32) -> InfoRes {
        Ok(Info {
            width: ImageDim::new(width)?,
            height: ImageDim::new(height)?,
            bit_depth: BitDepth::new(1).unwrap(),
            channels: ChannelDepth::new(1).expect("Bitmap channel depth"),
        })
    }

    /// Create a new info struct for PGM images.
    pub fn new_pgm(width: u32, height: u32, bit_depth: u16) -> InfoRes {
        Ok(Info {
            width: ImageDim::new(width)?,
            height: ImageDim::new(height)?,
            bit_depth: BitDepth::new(bit_depth)?,
            channels: ChannelDepth::new(1).expect("Greyscale channel depth"),
        })
    }

    /// Create a new info struct for PPM images.
    pub fn new_ppm(width: u32, height: u32, bit_depth: u16) -> InfoRes {
        Ok(Info {
            width: ImageDim::new(width)?,
            height: ImageDim::new(height)?,
            bit_depth: BitDepth::new(bit_depth)?,
            channels: ChannelDepth::new(3).expect("RGB24 channel depth"),
        })
    }

    /// Create a new info struct for PAM images.
    pub fn new_pam(width: u32, height: u32, bit_depth: u16, channels: u32) -> InfoRes {
        Ok(Info {
            width: ImageDim::new(width)?,
            height: ImageDim::new(height)?,
            bit_depth: BitDepth::new(bit_depth)?,
            channels: ChannelDepth::new(channels)?,
        })
    }

    /// Validate that u8 sample values agree with header info.
    pub fn validate_u8_samples(&self, samples: &[u8]) -> Result<(), NetpbmError> {
        // Check that the sample size is correct.
        self.validate_sample_size(samples.len())?;

        // Check samples against bit depth bound.
        for (offset, &sample) in samples.iter().enumerate() {
            if sample as u16 > self.bit_depth.value() {
                return Err(NetpbmError::OversizedSample {
                    offset,
                    bit_depth: self.bit_depth,
                });
            }
        }

        Ok(())
    }

    /// Validate that u16 sample values agree with header info.
    pub fn validate_u16_samples(&self, samples: &[u16]) -> Result<(), NetpbmError> {
        // Check that the sample size is correct.
        self.validate_sample_size(samples.len())?;

        // Check samples against bit depth bound.
        for (offset, &sample) in samples.iter().enumerate() {
            if sample > self.bit_depth.value() {
                return Err(NetpbmError::OversizedSample {
                    offset,
                    bit_depth: self.bit_depth,
                });
            }
        }

        Ok(())
    }

    // Validate that the number of samples corresponds to the image dimensions.
    fn validate_sample_size(&self, samples_len: usize) -> Result<(), NetpbmError> {
        let expected_samples = self.width.value() * self.height.value() * self.channels.value();
        if expected_samples != samples_len as u32 {
            return Err(NetpbmError::MalformedInitArray {
                data_size: samples_len,
                width: self.width,
                height: self.height,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_depth() {
        assert!(BitDepth::new(1).is_ok());
        assert!(BitDepth::new(255).is_ok());
        assert!(BitDepth::new(65535).is_ok());
        assert!(BitDepth::new(0).is_err());
    }

    #[test]
    fn test_image_dim() {
        assert!(ImageDim::new(1).is_ok());
        assert!(ImageDim::new(0).is_err());
        assert!(ImageDim::new(1000000).is_ok());
    }

    #[test]
    fn test_channel_depth() {
        assert!(ChannelDepth::new(1).is_ok());
        assert!(ChannelDepth::new(3).is_ok());
        assert!(ChannelDepth::new(0).is_err());
        assert!(ChannelDepth::new(100).is_ok());
    }
}
