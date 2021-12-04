
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const BITS: usize = Self::BYTES * u8::BITS as usize;
    pub const BYTES: usize = std::mem::size_of::<Self>();

    pub fn from_f32(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: (r * 255.999) as u8,
            g: (g * 255.999) as u8,
            b: (b * 255.999) as u8,
        }
    }

    pub fn as_u8(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}
