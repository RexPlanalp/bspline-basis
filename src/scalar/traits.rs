use core::ops::{Add, Div, Mul, Sub};
use num_traits::{One, Zero};

pub trait BSplineScalar:
    Copy
    + Zero
    + One
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialEq
{
    fn from_usize(n: usize) -> Self;
    fn from_f64(f: f64) -> Self;
    fn real(self) -> f64;
    fn imag(self) -> f64;
    fn conj(self) -> Self;
    fn abs_f64(self) -> f64;
}