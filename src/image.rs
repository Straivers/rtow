#[derive(Clone, Copy)]
pub enum Format {
    RgbU8,
    BgrU8,
}

impl Format {
    pub fn bits_per_pixel(&self) -> u8 {
        match self {
            Format::RgbU8 | Format::BgrU8 => 8,
        }
    }

    pub fn bytes_per_pixel(&self) -> u8 {
        match self {
            Self::RgbU8 | Format::BgrU8 => 24,
        }
    }
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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn bits_per_pixel(&self) -> u8 {
        self.format.bits_per_pixel()
    }

    pub fn bytes_per_pixel(&self) -> u8 {
        self.format.bytes_per_pixel()
    }

    pub fn line(&self, line: u32) -> &[u8] {
        let length = self.width as usize * self.bytes_per_pixel() as usize;
        let offset = line as usize * length;
        &self.buffer[offset..offset + length]
    }

    pub fn clone_as_format(&self, format: Format) -> Self {
        match self.format {
            Format::RgbU8 => match format {
                Format::RgbU8 => self.clone(),
                Format::BgrU8 => {
                    let mut clone = self.clone();
                    for pixel in clone.buffer.chunks_mut(3) {
                        pixel.swap(0, 2);
                    }
                    clone
                }
            },
            Format::BgrU8 => match format {
                Format::RgbU8 => {
                    let mut clone = self.clone();
                    for pixel in clone.buffer.chunks_mut(3) {
                        pixel.swap(0, 2);
                    }
                    clone
                }
                Format::BgrU8 => self.clone(),
            },
        }
    }
}
