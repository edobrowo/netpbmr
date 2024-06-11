use crate::{fields::*, NetpbmError, NetpbmFileFormat};
// use std::error::Error;
// use std::io::{BufWriter, Write};

/// PPM (Portable Pixel Map) image.
///
/// Each PPM image fundamentally consists of the image width,
/// the image height, the bit depth, and a sequence of rows of
/// color channel data. Each pixel is represented by a triplet
/// of color channel data (red, green, blue). There are `height`
/// number of rows, each with `width` color triplets.
///
/// Each PPM image also has associated with it a magic number.
/// which is either the bytes `P6` or `P3`. The magic number indicates
/// the PPM file format (see PpmFile for details). The file format
/// indicates how the PPM file is serialized.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PpmImage {
    colors: Vec<[u16; 3]>,
    width: ImageDim,
    height: ImageDim,
    bit_depth: BitDepth,
}

impl PpmImage {
    /// Create a new PPM image from the color channel values,
    /// image width, image height, and bit depth.
    pub fn from_colors(
        colors: Vec<[u16; 3]>,
        width: u32,
        height: u32,
        bit_depth: u32,
    ) -> Result<PpmImage, NetpbmError> {
        let width = ImageDim::new(width)?;
        let height = ImageDim::new(height)?;

        // The length of the color channel buffer should be
        // equal to the image width times the image height.
        if colors.len() as u32 != width.value() * height.value() {
            return Err(NetpbmError::MalformedInitArray {
                length: colors.len() as u32,
                width,
                height,
            });
        }

        let bit_depth = BitDepth::new(bit_depth)?;

        // All color channel values in the image must be less than the
        // given bit depth.
        for color in colors.iter() {
            if let Some(&channel) = color.iter().find(|&chan| *chan as u32 > bit_depth.value()) {
                return Err(NetpbmError::OversizedChannel { channel, bit_depth });
            }
        }

        Ok(PpmImage {
            colors,
            width,
            height,
            bit_depth,
        })
    }

    /// Create a new PPM image from the color channel values,
    /// image width, and image height. The bit depth is inferred
    /// from the maximum color channel value.
    pub fn with_inferred_bd(
        colors: Vec<[u16; 3]>,
        width: u32,
        height: u32,
    ) -> Result<PpmImage, NetpbmError> {
        let width = ImageDim::new(width)?;
        let height = ImageDim::new(height)?;

        // The length of the color channel buffer should be
        // equal to the image width times the image height.
        if colors.len() as u32 != width.value() * height.value() {
            return Err(NetpbmError::MalformedInitArray {
                length: colors.len() as u32,
                width,
                height,
            });
        }

        // The bit depth is the maximum color channel value.
        let bit_depth = *colors
            .iter()
            .map(|color| color.iter().max().unwrap())
            .max()
            .unwrap() as u32;
        let bit_depth = BitDepth::new(bit_depth)?;

        Ok(PpmImage {
            colors,
            width,
            height,
            bit_depth,
        })
    }

    /// Get a ref to the color triplet at position (x, y)
    /// in the image.
    pub fn get(&self, x: usize, y: usize) -> &[u16; 3] {
        &self.colors[y * self.width.value() as usize + x]
    }

    /// Iterate over the color triplets.
    pub fn iter(&self) -> core::slice::Iter<'_, [u16; 3]> {
        self.colors.iter()
    }
}

/// PPM (Portable Pixel Map) file.
///
/// PPM files can be one of two formats:
///
/// `raw` PPM files consist of a sequence of PPM images.
/// Color channel values are serialized as unsigned
/// binary integers.
/// The `raw` format uses the magic number `P6`.
///
/// `plain` PPM files consist of a single PPM image.
/// Color channel values are written as ASCII-encoded
/// decimal numbers.
/// The `plain` format uses the magic number `P3`.
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PpmFile {
    format: PpmFormat,
}

impl PpmFile {
    /// Create a new PPM `raw` file.
    pub fn new_raw() -> Self {
        PpmFile {
            format: PpmFormat::Raw(PpmRaw::new()),
        }
    }

    /// Create a new PPM `plain` file.
    pub fn new_plain(image: PpmImage) -> Self {
        PpmFile {
            format: PpmFormat::Plain(PpmPlain::new(image)),
        }
    }
}

/// Specifies the PPM format.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum PpmFormat {
    Raw(PpmRaw),
    Plain(PpmPlain),
}

/// PPM `raw` file format.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PpmRaw {
    images: Vec<PpmImage>,
}

impl PpmRaw {
    /// Make an empty PPM `raw` file.
    pub fn new() -> Self {
        PpmRaw { images: Vec::new() }
    }

    /// Add a new image to the image list.
    pub fn add_image(&mut self, image: PpmImage) {
        self.images.push(image)
    }

    /// Iterate over the contained PPM images.
    pub fn iter(&self) -> core::slice::Iter<'_, PpmImage> {
        self.images.iter()
    }
}

impl NetpbmFileFormat for PpmRaw {
    fn magic_number(&self) -> MagicNumber {
        MagicNumber::P6
    }
}

impl From<Vec<PpmImage>> for PpmRaw {
    /// Make a PPM `raw` file given a list of PPM images.
    fn from(images: Vec<PpmImage>) -> Self {
        PpmRaw { images }
    }
}

impl Default for PpmRaw {
    fn default() -> Self {
        Self::new()
    }
}

/// PPM `plain` file format.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PpmPlain {
    image: PpmImage,
}

impl PpmPlain {
    /// Make an empty PPM `plain` file.
    pub fn new(image: PpmImage) -> Self {
        PpmPlain { image }
    }

    /// Get a ref to the contained PPM image.
    fn image_ref(&self) -> &PpmImage {
        &self.image
    }
}

impl NetpbmFileFormat for PpmPlain {
    fn magic_number(&self) -> MagicNumber {
        MagicNumber::P3
    }
}

impl From<PpmImage> for PpmPlain {
    /// Make a PPM `plain` file given a single PPM image.
    fn from(image: PpmImage) -> Self {
        PpmPlain { image }
    }
}

// #[derive(Debug)]
// pub struct PpmWriter<W: Write> {
//     stream: BufWriter<W>,
// }

// impl<W: Write> PpmWriter<W> {
//     pub fn new(inner: W) -> PpmWriter<W> {
//         let stream = BufWriter::new(inner);
//         PpmWriter { stream }
//     }

//     pub fn write_all(&mut self, image: &PpmImage) -> Result<usize, Box<dyn Error>> {
//         let mut bytes = Vec::new();

//         let header = format!(
//             "{}\n{} {} {}\n",
//             MagicNumber::P6.value(),
//             image.width,
//             image.height,
//             image.bit_depth
//         );
//         bytes.extend_from_slice(header.as_bytes());

//         self.stream.write_all(&bytes)?;

//         bytes.clear();
//         for color in &image.colors {
//             // TODO: If bit depth is less than 256, 1 byte is used per channel. Otherwise 2 bytes is used, MSB first.
//             bytes.extend_from_slice(color);
//         }
//         self.stream.write_all(&bytes)?;

//         self.stream.flush()?;

//         Ok(0)
//     }

//     pub fn make_and_write_all(
//         &mut self,
//         data: Vec<[u8; 3]>,
//         width: u32,
//         height: u32,
//         bit_depth: u32,
//     ) -> Result<usize, Box<dyn Error>> {
//         let image = PpmImage::from_colors(data, width, height, bit_depth)?;
//         self.write_all(&image)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::PpmWriter;
//     use std::io;

//     /// Dummy buffer
//     #[derive(Debug)]
//     struct ImageBuffer {
//         buffer: Vec<u8>,
//     }

//     impl ImageBuffer {
//         fn new() -> Self {
//             ImageBuffer { buffer: Vec::new() }
//         }
//     }

//     impl io::Write for ImageBuffer {
//         fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
//             self.buffer.extend_from_slice(buf);
//             Ok(buf.len())
//         }

//         fn flush(&mut self) -> io::Result<()> {
//             Ok(())
//         }
//     }

//     #[test]
//     fn invalid_images() {
//         let data: Vec<[u8; 3]> = vec![
//             [255, 0, 0],
//             [0, 255, 0],
//             [0, 0, 255],
//             [255, 255, 0],
//             [255, 255, 255],
//             [0, 0, 0],
//         ];

//         let buffer = ImageBuffer::new();
//         let mut stream = PpmWriter::new(buffer);

//         assert!(!stream.make_and_write_all(data.clone(), 3, 0, 255).is_ok());
//         assert!(!stream.make_and_write_all(data.clone(), 0, 2, 255).is_ok());
//         assert!(!stream.make_and_write_all(data.clone(), 3, 3, 255).is_ok());
//         assert!(!stream.make_and_write_all(data.clone(), 2, 2, 255).is_ok());
//         assert!(!stream.make_and_write_all(data.clone(), 3, 2, 0).is_ok());
//         assert!(!stream.make_and_write_all(data.clone(), 3, 2, 65536).is_ok());
//         assert!(!stream
//             .make_and_write_all(data, u32::MAX, u32::MAX, 255)
//             .is_ok());
//     }

//     #[test]
//     fn valid_images() {
//         let data: Vec<[u8; 3]> = vec![
//             [255, 0, 0],
//             [0, 255, 0],
//             [0, 0, 255],
//             [255, 255, 0],
//             [255, 255, 255],
//             [0, 0, 0],
//         ];

//         let mut ppmwriter = PpmWriter::new(ImageBuffer::new());
//         let expected = [
//             80, 54, 10, 51, 32, 50, 32, 50, 53, 53, 10, 255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255,
//             0, 255, 255, 255, 0, 0, 0,
//         ];

//         assert!(ppmwriter.make_and_write_all(data, 3, 2, 255).is_ok());

//         let inner = ppmwriter.stream.into_inner().unwrap().buffer;
//         assert_eq!(inner[..], expected[..]);
//     }
// }
