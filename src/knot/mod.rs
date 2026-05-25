mod builders;
mod complex;
mod real;

pub use real::{RealKnotConfig, RealKnotVector};
pub use complex::{ComplexKnotConfig, ComplexKnotVector};

use crate::BSplineScalar;
use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write};
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

    fn dump(&self) -> std::io::Result<()> {
        let path = Path::new("output").join(self.outfile());
        let output_file = File::create(path)?;
        let mut writer = BufWriter::new(output_file);

        for x in self.knots() {
            writeln!(writer, "{} {}", x.real(), x.imag())?;
        }

        Ok(())
    }
}
