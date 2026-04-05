use std::f64::consts::PI;

use bspline_basis::bsplines::dump::dump_basis;
use bspline_basis::bsplines::{
    complex::{ComplexBSplineBasis, ComplexBSplineBasisConfig},
    real::BSplineBasisConfig,
};
use bspline_basis::knots::complex::EcsConfig;
fn main() {
    let ecs_config = EcsConfig {
        r0: 5.0,
        eta: PI / 4.0,
    };

    let bspline_basis_config = BSplineBasisConfig {
        start: 0.0,
        end: 10.0,
        n_basis: 30,
        order: 7,
    };

    let complex_bspline_basis_config = ComplexBSplineBasisConfig {
        config: bspline_basis_config,
        ecs_config,
    };

    let complex_basis = ComplexBSplineBasis::new(complex_bspline_basis_config);

    dump_basis(&complex_basis, 1000).expect("Should dump");
}
