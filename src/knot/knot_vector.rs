// Internal Imports
use crate::scalar::BSplineScalar;
use crate::config::Config;

// External Imports
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub trait KnotVector {
    type Scalar: BSplineScalar;
    type Configuration: Config;

    fn knots(&self) -> &[Self::Scalar];
    fn config(&self) -> &Self::Configuration;

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

    fn parameter_domain(&self) -> (f64, f64);

    fn parameter_to_knot(&self, x: f64) -> Self::Scalar;
    
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
