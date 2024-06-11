use crate::NetpbmError;
use std::fmt;

/// Magic number field.
///
/// Each netpbm format has an assigned magic number. Every
/// netpbm magic number consists of the two bytes
/// `PN`, where N is a natural number in ASCII.
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
/// netpbm specifies that some formats must specify the
/// maximum color channel value in an image (i.e., bit depth).
/// The bit depth must be between 1 and 65535 inclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BitDepth(u32);

impl BitDepth {
    pub const BIT_DEPTH_MIN: u32 = 1;
    pub const BIT_DEPTH_MAX: u32 = 65535;

    /// Create a new BitDepth from a u32.
    pub fn new(val: u32) -> Result<Self, NetpbmError> {
        if (Self::BIT_DEPTH_MIN..=Self::BIT_DEPTH_MAX).contains(&val) {
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
/// Image dimensions in netpbm must be non-negative. There is
/// no indication of maximum value, so the 32-bit unsigned
/// bound is used.
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
