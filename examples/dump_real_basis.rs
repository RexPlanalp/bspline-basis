use bspline_basis::{BasisConfig, RealBSplineBasis};

fn main() -> std::io::Result<()> {
    let config = BasisConfig {
        n_basis: 30,
        order: 7,
    };
    let real_basis = RealBSplineBasis::build(&config, 0.0, 10.0).unwrap();

    real_basis.dump(1000)?;

    Ok(())
}
