use crate::NetpbmError;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BitDepth(u32);

impl BitDepth {
    pub const BITDEPTH_MIN: u32 = 1;
    pub const BITDEPTH_MAX: u32 = 65535;

    pub fn new(val: u32) -> Result<Self, NetpbmError> {
        if (Self::BITDEPTH_MIN..=Self::BITDEPTH_MAX).contains(&val) {
            Ok(Self(val))
        } else {
            Err(NetpbmError::InvalidBitDepth { value: val })
        }
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for BitDepth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImageDim(u32);

impl ImageDim {
    pub fn new(val: u32) -> Result<Self, NetpbmError> {
        if val > 0 {
            Ok(Self(val))
        } else {
            Err(NetpbmError::InvalidImageDim { value: val })
        }
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for ImageDim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
