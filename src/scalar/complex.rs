use super::traits::BSplineScalar;

use num_complex::Complex64;

impl BSplineScalar for Complex64 {
    fn from_usize(n: usize) -> Self {
        Complex64::new(n as f64, 0.0)
    }
    fn from_f64(f: f64) -> Self {
        Complex64::new(f, 0.0)
    }
    fn real(self) -> f64 {
        self.re
    }
    fn imag(self) -> f64 {
        self.im
    }
    fn conj(self) -> Complex64 {
        Complex64::new(self.re, -self.im)
    }
    fn abs_f64(self) -> f64 {
        self.norm()
    }
}
