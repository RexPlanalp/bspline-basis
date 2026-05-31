use bspline_basis::{BasisConfig, RealBSplineBasis};
use num_complex::ComplexFloat;

fn main() -> std::io::Result<()> {
    let config = BasisConfig {
        n_basis: 30,
        order: 7,
    };
    let real_basis = RealBSplineBasis::build(&config, 0.0, 10.0).unwrap();

    real_basis.dump(1000)?;

    let integrator = real_basis.integrator();
    let val = integrator.integrate_kinetic(15, 15);

    println!("{}+{}j", val.re(), val.im());

    Ok(())
}
