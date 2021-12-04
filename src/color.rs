#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_f32(r: f32, g: f32, b: f32) -> Color {
        Color {
            r: (r * 255.999) as u8,
            g: (g * 255.999) as u8,
            b: (b * 255.999) as u8,
        }
    }
}
