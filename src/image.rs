
use crate::color::Color;

/// Stored in row-order from bottom-to-top.
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<Color>,
}

impl Image {
    pub fn new(width: u32, height: u32, buffer: Vec<Color>) -> Self {
        Self {
            width,
            height,
            buffer
        }
    }

    pub fn bits_per_pixel(&self) -> u8 {
        24
    }

    pub fn bytes_per_pixel(&self) -> u8 {
        3
    }
}
