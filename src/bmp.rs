//! Code for dencoding .bmp files.
//!

use std::io::Write;

use crate::image::{Format as ImageFormat, Image};

const PIXELS_PER_METER_72_DPI: i32 = 2835;

/// File header for BMP-encoded files.
#[repr()]
#[derive(Clone, Copy, Debug)]
struct Header {
    magic: [u8; 2],
    /// The size of the image in bytes.
    file_size: u32,
    /// Must be 0
    reserved0: u16,
    /// Must be 0
    reserved1: u16,
    /// The byte index at which the image data starts.
    image_offset: u32,
}

impl Header {
    const FILE_SIZE: u32 = 14;

    /// Generates the appropriate file header for an image using [`InfoHeader`].
    fn for_info_header(info: &InfoHeader) -> Self {
        let total_header_size = Self::FILE_SIZE + InfoHeader::FILE_SIZE;
        Self {
            magic: [0x42, 0x4D],
            file_size: info.image_size + total_header_size,
            reserved0: 0,
            reserved1: 0,
            image_offset: total_header_size,
        }
    }

    /// Writes the header to the writer in packed form.
    fn write_to_buffer(&self, writer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        writer.write_all(&self.magic)?;
        writer.write_all(as_u8_slice(&self.file_size))?;
        writer.write_all(as_u8_slice(&self.reserved0))?;
        writer.write_all(as_u8_slice(&self.reserved1))?;
        writer.write_all(as_u8_slice(&self.image_offset))?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct InfoHeader {
    /// The size of this header in bytes, without padding.
    header_size: u32,
    /// The width of the image in pixels.
    image_width: i32,
    /// The height of the image in pixels, can be negative for top-to-bottom
    /// images.
    image_height: i32,
    /// The number of color planes in this image. Always 1.
    color_planes: u16,
    /// The number of bits per pixel, can be 1, 4, 8, 16, 24, or 32.
    bits_per_pixel: u16,
    /// The way the image was compressed.
    compression_method: CompressionMethod,
    /// The size of the image in bytes, can be 0 for CompressionMethod::BI_RGB.
    image_size: u32,
    /// The horizontal resolution of the image in pixels per meter. Defaults to
    /// 72 DPI (2834.6472 pixels/meter).
    resolution_x: i32,
    /// The vertical resolution of the image in pixels per meter. Defaults to 72
    /// DPI (2834.6472 pixels/meter).
    resolution_y: i32,
    /// The size of the color palette in when using an indexed color compression
    /// mode.
    palette_size: u32,
    /// The number of 'important' colors in the palette. 0 means all colors are
    /// 'important'.
    num_important_colors: u32,
}

impl InfoHeader {
    const FILE_SIZE: u32 = std::mem::size_of::<Self>() as u32;

    pub fn new(image: &Image) -> Self {
        let bytes_per_line = image.line(0).len() as u32;
        let image_size = (bytes_per_line + (bytes_per_line % 4)) * image.height();

        Self {
            header_size: InfoHeader::FILE_SIZE,
            image_width: image.width().try_into().unwrap(),
            image_height: image.height().try_into().unwrap(),
            color_planes: 1,
            bits_per_pixel: image.bits_per_pixel().into(),
            compression_method: CompressionMethod::BI_RGB,
            image_size,
            resolution_x: PIXELS_PER_METER_72_DPI,
            resolution_y: PIXELS_PER_METER_72_DPI,
            palette_size: 0,
            num_important_colors: 0,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
enum CompressionMethod {
    /// No compression
    BI_RGB = 0,
}

fn as_u8_slice<T: Sized>(o: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts((o as *const T).cast(), std::mem::size_of::<T>()) }
}

pub fn encode(image: &Image, buffer: &mut Vec<u8>) {
    let pixels = image.clone_as_format(ImageFormat::BgrU8);

    let align_bytes: [u8; 4] = [0, 0, 0, 0];
    let align = image.width() as usize % 4;

    let info_header = InfoHeader::new(image);
    let header = Header::for_info_header(&info_header);

    header.write_to_buffer(buffer).unwrap();
    buffer.write_all(as_u8_slice(&info_header)).unwrap();

    for y in 0..image.height() {
        buffer.write_all(pixels.line(y)).unwrap();
        buffer.write_all(&align_bytes[0..align]).unwrap();
    }
}
