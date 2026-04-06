use super::scalar::BSplineScalar;
pub trait KnotVector {
    type Scalar: BSplineScalar;

    fn knots(&self) -> &[Self::Scalar];

    fn outfile(&self) -> &'static str {
        "knots.txt"
    }

    fn interval(&self, i: usize) -> (Self::Scalar, Self::Scalar) {
        (self.knots()[i], self.knots()[i + 1])
    }

    fn in_interval(&self, x: Self::Scalar, i: usize) -> bool {
        let (start, end) = self.interval(i);
        x.real() >= start.real() && x.real() < end.real()
    }

    fn start(&self) -> f64;
    fn end(&self) -> f64;
}
