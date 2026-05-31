use std::f64::consts::PI;
use bspline_basis::{BasisConfig, ComplexBSplineBasis};

fn main() -> std::io::Result<()> {
    let config = BasisConfig {
        n_basis: 30,
        order: 7,
    };
    let complex_basis = ComplexBSplineBasis::build(&config, 0.0, 10.0, PI / 4.0, 5.0).unwrap();

    complex_basis.dump(1000)?;

    Ok(())
}
