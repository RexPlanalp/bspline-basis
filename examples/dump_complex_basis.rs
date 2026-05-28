use std::f64::consts::PI;
use std::fs::{File, create_dir_all};
use std::io::{BufWriter, Write};

use bspline_basis::{BasisConfig, ComplexBSplineBasis, KnotVector};
use ndarray::linspace;

fn main() -> std::io::Result<()> {
    create_dir_all("output")?;

    let config = BasisConfig {
        n_basis: 30,
        order: 7,
    };
    let complex_basis = ComplexBSplineBasis::build(&config, 0.0, 10.0, PI / 4.0, 5.0).unwrap();

    complex_basis.dump(1000)?;

    Ok(())
}
