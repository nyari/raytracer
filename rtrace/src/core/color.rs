use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};
use std::cmp::{Ordering};

use defs::{DefNumType, DefComplexType};
use tools::CompareWithTolerance;

use num::traits::{Zero, One};

#[derive(Debug, Clone, Copy)]
pub struct ColorBase<T> {
    r: T,
    g: T,
    b: T,
}

impl<T: Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Zero + One> ColorBase<T> {
    pub fn new(r: T, g: T, b: T) -> Self {
        Self {r: r,
              g: g,
              b: b}
    }

    pub fn get(&self) -> (T, T, T) {
        (self.r, self.g, self.b)
    }

    pub fn equal_eps(&self, other: &ColorBase<T>) -> bool {
        self.r.compare_eps(&other.r) == Ordering::Equal && 
        self.g.compare_eps(&other.g) == Ordering::Equal && 
        self.b.compare_eps(&other.b) == Ordering::Equal
    }

    pub fn normalize(&mut self) {
        self.r = self.r.min(T::one());
        self.g = self.g.min(T::one());
        self.b = self.b.min(T::one());
    }

    pub fn normalized(&self) -> ColorBase<T> {
        let mut result = self.clone();
        result.normalize();
        result
    }

    pub fn mul_scalar(&self, other: &T) -> Self {
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

    pub fn avg_intensity(&self) -> T {
        (self.r + self.g + self.b) / (3.0 * T::one())
    }

    pub fn scale_normalized(&self) -> Self {
        let maximum = self.r.max(self.g.max(self.b));
        if maximum > T::one() {
            Self {  r: self.r / maximum,
                    g: self.g / maximum,
                    b: self.b / maximum
            }
        } else {
            *self
        }
    }

    pub fn zero() -> Self {
        Self { r: T::zero(),
               g: T::zero(),
               b: T::zero(),
        }
    }

    pub fn one() -> Self {
        Self { r: T::one(),
               g: T::one(),
               b: T::one(),
        }
    }
}

impl<T: Add<Output=T>> Add for ColorBase<T> {
    type Output = ColorBase<T>;

    fn add(self, other: ColorBase<T>) -> ColorBase<T> {
        Self {  r: self.r + other.r,
                g: self.g + other.g,
                b: self.b + other.b }
    }
}

impl<T: AddAssign> AddAssign for ColorBase<T> {
    fn add_assign(&mut self, other: ColorBase<T>) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl<T: Sub<Output=T>> Sub for ColorBase<T> {
    type Output = ColorBase<T>;

    fn sub(self, other: ColorBase<T>) -> ColorBase<T> {
        Self {r: self.r - other.r,
               g: self.g - other.g,
               b: self.b - other.b}
    }
}
impl<T: SubAssign> SubAssign for ColorBase<T> {

    fn sub_assign(&mut self, other: ColorBase<T>) {
        self.r -= other.r;
        self.g -= other.g;
        self.b -= other.b;
    }
}

impl<T: Mul<Output=T>> Mul for ColorBase<T> {
    type Output = ColorBase<T>;

    fn mul(self, other: ColorBase<T>) -> ColorBase<T> {
        Self {r: self.r * other.r,
               g: self.g * other.g,
               b: self.b * other.b}
    }
}

impl<T: MulAssign> MulAssign for ColorBase<T> {
    fn mul_assign(&mut self, other: ColorBase<T>) {
        self.r *= other.r;
        self.g *= other.g;
        self.b *= other.b;
    }
}

pub type Color = ColorBase<DefNumType>;
pub type FresnelIndex = ColorBase<DefComplexType>;