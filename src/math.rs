use std::ops::{Add, Div, Mul, Neg, Sub};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct float3([f32; 3]);

impl float3 {
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

impl Add<f32> for float3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self([self.0[0] + rhs, self.0[1] + rhs, self.0[2] + rhs])
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

impl Mul<f32> for float3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
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

impl Div<f32> for float3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self([self.0[0] / rhs, self.0[1] / rhs, self.0[2] / rhs])
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point([f32; 3]);

impl Point {
    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn z(&self) -> f32 {
        self.0[2]
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
