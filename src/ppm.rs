use crate::fields::*;
use crate::NetpbmError;
use std::error::Error;
use std::io::{BufWriter, Write};

pub struct PpmImage {
    data: Vec<[u8; 3]>,
    width: ImageDim,
    height: ImageDim,
    bitdepth: BitDepth,
}

impl PpmImage {
    const MAGIC_NUMBER: &'static str = "P6";

    pub fn new(
        data: Vec<[u8; 3]>,
        width: u32,
        height: u32,
        bitdepth: u32,
    ) -> Result<PpmImage, NetpbmError> {
        let width = ImageDim::new(width)?;
        let height = ImageDim::new(height)?;
        let bitdepth = BitDepth::new(bitdepth)?;

        if data.len() as u32 != width.get() * height.get() {
            return Err(NetpbmError::MalformedInitArray {
                length: data.len() as u32,
                width,
                height,
            });
        }

        for color in data.iter() {
            if let Some(&channel) = color.iter().find(|&chan| *chan as u32 > bitdepth.get()) {
                return Err(NetpbmError::OversizedChannel { channel, bitdepth });
            }
        }

        Ok(PpmImage {
            data,
            width,
            height,
            bitdepth,
        })
    }
}

#[derive(Debug)]
pub struct PpmWriter<W: Write> {
    stream: BufWriter<W>,
}

impl<W: Write> PpmWriter<W> {
    pub fn new(inner: W) -> PpmWriter<W> {
        let stream = BufWriter::new(inner);
        PpmWriter { stream }
    }

    pub fn write_all(&mut self, image: &PpmImage) -> Result<usize, Box<dyn Error>> {
        let mut bytes = Vec::new();

        let header = format!(
            "{}\n{} {} {}\n",
            PpmImage::MAGIC_NUMBER,
            image.width,
            image.height,
            image.bitdepth
        );
        bytes.extend_from_slice(header.as_bytes());

        self.stream.write_all(&bytes)?;

        bytes.clear();
        for color in &image.data {
            // TODO: If bit depth is less than 256, 1 byte is used per channel. Otherwise 2 bytes is used, MSB first.
            bytes.extend_from_slice(color);
        }
        self.stream.write_all(&bytes)?;

        self.stream.flush()?;

        Ok(0)
    }

    pub fn make_and_write_all(
        &mut self,
        data: Vec<[u8; 3]>,
        width: u32,
        height: u32,
        bitdepth: u32,
    ) -> Result<usize, Box<dyn Error>> {
        let image = PpmImage::new(data, width, height, bitdepth)?;
        self.write_all(&image)
    }
}

#[cfg(test)]
mod tests {
    use super::PpmWriter;
    use std::io;

    // Dummy buffer used to validate successful writes
    #[derive(Debug)]
    struct ImageBuffer {
        buffer: Vec<u8>,
    }

    impl ImageBuffer {
        fn new() -> Self {
            ImageBuffer { buffer: Vec::new() }
        }
    }

    impl io::Write for ImageBuffer {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn invalid_images() {
        let data: Vec<[u8; 3]> = vec![
            [255, 0, 0],
            [0, 255, 0],
            [0, 0, 255],
            [255, 255, 0],
            [255, 255, 255],
            [0, 0, 0],
        ];

        let buffer = ImageBuffer::new();
        let mut stream = PpmWriter::new(buffer);

        assert!(!stream.make_and_write_all(data.clone(), 3, 0, 255).is_ok());
        assert!(!stream.make_and_write_all(data.clone(), 0, 2, 255).is_ok());
        assert!(!stream.make_and_write_all(data.clone(), 3, 3, 255).is_ok());
        assert!(!stream.make_and_write_all(data.clone(), 2, 2, 255).is_ok());
        assert!(!stream.make_and_write_all(data.clone(), 3, 2, 0).is_ok());
        assert!(!stream.make_and_write_all(data.clone(), 3, 2, 65536).is_ok());
        assert!(!stream
            .make_and_write_all(data, u32::MAX, u32::MAX, 255)
            .is_ok());
    }

    #[test]
    fn valid_images() {
        let data: Vec<[u8; 3]> = vec![
            [255, 0, 0],
            [0, 255, 0],
            [0, 0, 255],
            [255, 255, 0],
            [255, 255, 255],
            [0, 0, 0],
        ];

        let mut ppmwriter = PpmWriter::new(ImageBuffer::new());
        let expected = [
            80, 54, 10, 51, 32, 50, 32, 50, 53, 53, 10, 255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255,
            0, 255, 255, 255, 0, 0, 0,
        ];

        assert!(ppmwriter.make_and_write_all(data, 3, 2, 255).is_ok());

        let inner = ppmwriter.stream.into_inner().unwrap().buffer;
        assert_eq!(inner[..], expected[..]);
    }
}
