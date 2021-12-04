#[repr(C)]
#[derive(Clone, Copy)]
pub struct RgbU8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbU8 {
    pub fn as_u8(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

impl From<&RgbF32> for RgbU8 {
    fn from(rgb_f32: &RgbF32) -> Self {
        Self {
            r: (rgb_f32.r.clamp(0.0, 1.0) * 255.999) as u8,
            g: (rgb_f32.g.clamp(0.0, 1.0) * 255.999) as u8,
            b: (rgb_f32.b.clamp(0.0, 1.0) * 255.999) as u8,
        }
    }
}

pub struct RgbF32 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl RgbF32 {
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0);

    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

impl std::ops::Div<f32> for RgbF32 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl std::ops::AddAssign for RgbF32 {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl std::ops::DivAssign<f32> for RgbF32 {
    fn div_assign(&mut self, rhs: f32) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}
