use crate::color::Rgb;

#[derive(Clone, Copy)]
pub enum Format {
    RGB8,
    BGR8,
}

/// Stored in row-order from bottom-to-top.
#[derive(Clone)]
pub struct Image {
    width: u32,
    height: u32,
    format: Format,
    buffer: Vec<u8>,
}

impl Image {
    pub fn new(width: u32, height: u32, format: Format, buffer: Vec<u8>) -> Self {
        Self {
            width,
            height,
            format,
            buffer,
        }
    }

    pub fn size(&self) -> usize {
        self.buffer.len()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn bits_per_pixel(&self) -> u8 {
        Rgb::BITS as u8
    }

    pub fn bytes_per_pixel(&self) -> u8 {
        Rgb::BYTES as u8
    }

    pub fn line(&self, line: u32) -> &[u8] {
        let length = self.width as usize * self.bytes_per_pixel() as usize;
        let offset = line as usize * length;
        &self.buffer[offset..offset + length]
    }

    pub fn clone_as_format(&self, format: Format) -> Self {
        match self.format {
            Format::RGB8 => match format {
                Format::RGB8 => self.clone(),
                Format::BGR8 => {
                    let mut clone = self.clone();
                    for pixel in clone.buffer.chunks_mut(3) {
                        pixel.swap(0, 2);
                    }
                    clone
                }
            },
            Format::BGR8 => match format {
                Format::RGB8 => {
                    let mut clone = self.clone();
                    for pixel in clone.buffer.chunks_mut(3) {
                        pixel.swap(0, 2);
                    }
                    clone
                }
                Format::BGR8 => self.clone(),
            },
        }
    }
}
