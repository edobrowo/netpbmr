use header::*;
use std::error::Error;
use std::fmt;
use std::io;

pub mod formats;
pub mod header;
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

/// Indicates how to encode a PNM file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EncodingType {
    /// Sample data is serialized as bytes.
    Raw,

    /// Sample data is written as ASCII integers separated
    /// by whitespace.
    Plain,
}

/// Generalizes over u8 and u16 since netpbm permits
/// samples to be either 8- or 16-bit.
pub trait SampleType {
    /// The sample type, either u8 or u16.
    type Sample;

    /// Validate that the samples agree with the header info.
    fn validate_samples(info: &Info, samples: &[Self::Sample]) -> Result<(), NetpbmError>;

    /// Convert the Sample slice into a SampleBuffer.
    fn to_sample_buffer(samples: &[Self::Sample]) -> SampleBuffer;
}

impl SampleType for u8 {
    type Sample = u8;

    fn validate_samples(info: &Info, samples: &[Self::Sample]) -> Result<(), NetpbmError> {
        info.validate_u8_samples(samples)
    }

    fn to_sample_buffer(samples: &[Self::Sample]) -> SampleBuffer {
        SampleBuffer::EIGHT(samples)
    }
}

impl SampleType for u16 {
    type Sample = u16;

    fn validate_samples(info: &Info, samples: &[Self::Sample]) -> Result<(), NetpbmError> {
        info.validate_u16_samples(samples)
    }

    fn to_sample_buffer(samples: &[Self::Sample]) -> SampleBuffer {
        SampleBuffer::SIXTEEN(samples)
    }
}

/// Convenience sample buffer.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SampleBuffer<'a> {
    EIGHT(&'a [u8]),
    SIXTEEN(&'a [u16]),
}

#[derive(Debug)]
pub struct Writer<W: io::Write> {
    writer: W,
    image_info: Vec<Info>,
}

impl<W: io::Write> Writer<W> {
    /// Make a new netpbm writer.
    pub fn new(writer: W) -> Result<Self, NetpbmError> {
        Ok(Self {
            writer,
            image_info: Vec::new(),
        })
    }

    /// Retrieve a ref to the image info.
    pub fn info(&self) -> &Vec<Info> {
        &self.image_info
    }

    // TODO : to support comments, another, more granular constructor
    // is required. It should allow specifying where comments go
    // with regard to header fields.
}

//
//     // WRITE
//     let path = Path::new(r"/path/to/image.png");
//     let file = File::create(path).unwrap();
//     let ref mut writer = BufWriter::new(file);
//
//     let mut encoder = netpbm::PPMEncoder::new(writer, width, height, bit_depth);

//     let data = [255, 0, 0, 255, 0, 0, 0, 255];
//     encoder.write_image_data(&data).unwrap();
//     encoder.write_tuple((u8, u8, u8)); // only if PPM, PAM
//     encoder.write_value(u8); // only if PBM, PGM
//
//

//     // READ
//     let decoder = netpbm::Decoder::new(File::open(r"path/to/image").unwrap());
//     let mut reader = decoder.reader().unwrap(); // reads the header
//
//     let mut buf = vec![0; reader.output_buffer_size()];
//
//     let info = reader.next_image(&mut buf).unwrap();
//     let bytes = &buf[..info.buffer_size()];
//
//     let bit_depth = reader.info().bit_depth;
//     let width = reader.info().width;
//     let height = reader.info().height;
//     let num_channels = reader.info().num_channels;
//
