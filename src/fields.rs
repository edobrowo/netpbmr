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

impl MagicNumber {
    /// Get the corresponding magic number bytes.
    pub fn value(&self) -> &'static str {
        match self {
            Self::P1 => "P1",
            Self::P2 => "P2",
            Self::P3 => "P3",
            Self::P4 => "P4",
            Self::P5 => "P5",
            Self::P6 => "P6",
            Self::P7 => "P7",
        }
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
