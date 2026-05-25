use std::f64::consts::PI;
use bspline_basis::{ComplexKnotConfig, ComplexKnotVector, KnotVector};
fn main() {
    let knot_config = ComplexKnotConfig {
        n_knots: 30,
        multiplicity: 3,
        start: 0.0,
        end: 10.0,
        eta: PI / 4.0,
        r0: 5.0
    };

    let real_knots = ComplexKnotVector::build(&knot_config).unwrap().dump();
}
