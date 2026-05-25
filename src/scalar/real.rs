use crate::BSplineScalar;

impl BSplineScalar for f64 {
    fn from_usize(n: usize) -> Self {
        n as f64
    }
    fn from_f64(f: f64) -> Self {
        f
    }
    fn real(self) -> f64 {
        self
    }
    fn imag(self) -> f64 {
        0.0
    }
    fn conj(self) -> f64 {
        self
    }
    fn abs_f64(self) -> f64 {
        self.abs()
    }
}
