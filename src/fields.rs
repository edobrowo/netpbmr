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
pub struct BitDepth(u32);

impl BitDepth {
    pub const MIN: u32 = 1;
    pub const MAX: u32 = 65535;

    /// Create a new BitDepth from a u32.
    pub fn new(val: u32) -> Result<Self, NetpbmError> {
        if (Self::MIN..=Self::MAX).contains(&val) {
            Ok(Self(val))
        } else {
            Err(NetpbmError::InvalidBitDepth { value: val })
        }
    }

    /// Get the BitDepth as a u32.
    pub fn value(&self) -> u32 {
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
    pub fn new(val: u32) -> Result<Self, NetpbmError> {
        if val > 0 {
            Ok(Self(val))
        } else {
            Err(NetpbmError::InvalidImageDim { value: val })
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
    pub fn new(val: u32) -> Result<Self, NetpbmError> {
        if val > 0 {
            Ok(Self(val))
        } else {
            Err(NetpbmError::InvalidChannelDepth { value: val })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_depth() {
        assert!(BitDepth::new(1).is_ok());
        assert!(BitDepth::new(255).is_ok());
        assert!(BitDepth::new(65535).is_ok());
        assert!(BitDepth::new(0).is_err());
        assert!(BitDepth::new(100000).is_err());
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
