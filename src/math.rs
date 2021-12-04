use std::ops::{Add, Div, DivAssign, Mul, Neg, Sub};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct float3([f32; 3]);

impl float3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn z(&self) -> f32 {
        self.0[2]
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.0[0] * self.0[0] + self.0[1] * self.0[1] + self.0[2] * self.0[2]
    }

    pub fn normalize(&mut self) {
        *self /= self.length();
    }

    /// Returns a normalized version of this vector.
    pub fn normalized(&self) -> Self {
        let mut other = *self;
        other.normalize();
        other
    }

    pub fn dot(&self, rhs: Self) -> f32 {
        self.0[0] * rhs.0[0] + self.0[1] * rhs.0[1] + self.0[2] * rhs.0[2]
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self([
            self.0[1] * rhs.0[2] - self.0[2] * rhs.0[1],
            self.0[2] * rhs.0[0] - self.0[0] * rhs.0[2],
            self.0[0] * rhs.0[1] - self.0[1] * rhs.0[0],
        ])
    }
}

impl Neg for float3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self([-self.0[0], -self.0[1], -self.0[2]])
    }
}

impl Add for float3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

impl Add<&Self> for float3 {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self {
        Self([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

impl Add<f32> for float3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self([self.0[0] + rhs, self.0[1] + rhs, self.0[2] + rhs])
    }
}

impl Add<float3> for f32 {
    type Output = float3;

    fn add(self, rhs: float3) -> Self::Output {
        rhs + self
    }
}

impl Sub for float3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl Sub<&Self> for float3 {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self {
        Self([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl Sub<f32> for float3 {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self {
        Self([self.0[0] - rhs, self.0[1] - rhs, self.0[2] - rhs])
    }
}

impl Mul for float3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
        ])
    }
}

impl Mul<&Self> for float3 {
    type Output = Self;
    fn mul(self, rhs: &Self) -> Self {
        Self([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
        ])
    }
}

impl Mul<f32> for float3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
    }
}

impl Mul<float3> for f32 {
    type Output = float3;
    fn mul(self, rhs: float3) -> Self::Output {
        rhs * self
    }
}

impl Div for float3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self([
            self.0[0] / rhs.0[0],
            self.0[1] / rhs.0[1],
            self.0[2] / rhs.0[2],
        ])
    }
}

impl Div<&Self> for float3 {
    type Output = Self;
    fn div(self, rhs: &Self) -> Self {
        Self([
            self.0[0] / rhs.0[0],
            self.0[1] / rhs.0[1],
            self.0[2] / rhs.0[2],
        ])
    }
}

impl Div<f32> for float3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self([self.0[0] / rhs, self.0[1] / rhs, self.0[2] / rhs])
    }
}

impl DivAssign<f32> for float3 {
    fn div_assign(&mut self, rhs: f32) {
        self.0[0] /= rhs;
        self.0[1] /= rhs;
        self.0[2] /= rhs;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point([f32; 3]);

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn z(&self) -> f32 {
        self.0[2]
    }

    pub fn as_vec(&self) -> float3 {
        float3(self.0)
    }
}

impl Add<float3> for Point {
    type Output = Self;
    fn add(self, rhs: float3) -> Self {
        Self([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

impl Sub<float3> for Point {
    type Output = Self;
    fn sub(self, rhs: float3) -> Self {
        Self([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl Sub<&float3> for Point {
    type Output = Self;
    fn sub(self, rhs: &float3) -> Self {
        Self([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl Mul<float3> for Point {
    type Output = Self;
    fn mul(self, rhs: float3) -> Self {
        Self([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
        ])
    }
}

impl Mul<&float3> for Point {
    type Output = Self;
    fn mul(self, rhs: &float3) -> Self {
        Self([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
        ])
    }
}

impl Div<float3> for Point {
    type Output = Self;
    fn div(self, rhs: float3) -> Self {
        Self([
            self.0[0] / rhs.0[0],
            self.0[1] / rhs.0[1],
            self.0[2] / rhs.0[2],
        ])
    }
}

impl Div<&float3> for Point {
    type Output = Self;
    fn div(self, rhs: &float3) -> Self {
        Self([
            self.0[0] / rhs.0[0],
            self.0[1] / rhs.0[1],
            self.0[2] / rhs.0[2],
        ])
    }
}
