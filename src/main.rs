mod bmp;
mod color;
mod image;

fn main() {
    let width = 256;
    let height = 256;
    let mut pixels = Vec::new();

    for j in 0..height {
        for i in 0..width {
            pixels.extend_from_slice(&color::Rgb::from_f32(
                i as f32 / (width as f32 - 1.0),
                j as f32 / (height as f32 - 1.0),
                0.25,
            ).as_u8());
        }
    }

    let image = image::Image::new(width, height, image::Format::RGB8, pixels);
    let mut buffer = Vec::new();
    bmp::encode(&image, &mut buffer);

    std::fs::write("image.bmp", buffer).unwrap();
}
