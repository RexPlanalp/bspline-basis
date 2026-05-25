use std::f64::consts::PI;
use bspline_basis::{ComplexBSplineBasis, BasisConfig};
fn main() {

    let config = BasisConfig{
        n_basis: 30,
        order: 7
    };
    let complex_basis = ComplexBSplineBasis::build(&config, 0.0, 10.0, PI / 4.0, 5.0).unwrap();

    let val = complex_basis.evaluator().db(15, 8.0);

    println!("Real: {}, Imag: {}", val.re, val.im);
}
