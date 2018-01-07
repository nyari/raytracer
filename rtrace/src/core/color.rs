use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};
use std::cmp::{Ordering};

use defs::FloatType;
use tools::CompareWithTolerance;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: FloatType,
    g: FloatType,
    b: FloatType,
}

impl Color {
    pub fn new(r: FloatType, g: FloatType, b: FloatType) -> Self {
        Self {r: r,
              g: g,
              b: b}
    }

    pub fn get(&self) -> (FloatType, FloatType, FloatType) {
        (self.r, self.g, self.b)
    }

    pub fn equal_eps(&self, other: &Color) -> bool {
        self.r.compare_eps(&other.r) == Ordering::Equal && 
        self.g.compare_eps(&other.g) == Ordering::Equal && 
        self.b.compare_eps(&other.b) == Ordering::Equal
    }

    pub fn normalize(&mut self) {
        self.r = self.r.min(1.0);
        self.g = self.g.min(1.0);
        self.b = self.b.min(1.0);
    }

    pub fn normalized(&self) -> Color {
        let mut result = self.clone();
        result.normalize();
        result
    }

    pub fn mul_scalar(&self, other: &FloatType) -> Self {
        Self {  r: self.r * other,
                g: self.g * other,
                b: self.b * other}
    }

    pub fn recip(&self) -> Self {
        Self {  r: self.r.recip(),
                g: self.g.recip(),
                b: self.b.recip()
            }
    }

    pub fn intensity_avg(&self) -> FloatType {
        (self.r + self.g + self.b) / 3.0
    }

    pub fn zero() -> Self {
        Self { r: 0.0,
               g: 0.0,
               b: 0.0,
        }
    }

    pub fn one() -> Self {
        Self { r: 1.0,
               g: 1.0,
               b: 1.0,
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Self {  r: self.r + other.r,
                g: self.g + other.g,
                b: self.b + other.b }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Self {r: self.r - other.r,
               g: self.g - other.g,
               b: self.b - other.b}
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, other: Color) {
        self.r -= other.r;
        self.g -= other.g;
        self.b -= other.b;
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Self {r: self.r * other.r,
               g: self.g * other.g,
               b: self.b * other.b}
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, other: Color) {
        self.r *= other.r;
        self.g *= other.g;
        self.b *= other.b;
    }
}

pub type FresnelIndex = Color;