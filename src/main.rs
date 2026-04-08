use bspline_basis::config::basis::BasisConfig;
use bspline_basis::basis::real::RealBSplineBasis;
use bspline_basis::basis::dump_basis::BasisDump;
use bspline_basis::core::dump::Dump;

fn main() {
    let basis_config = BasisConfig {
        n_basis: 30,
        order: 7,
        start: 0.0,
        end: 10.0
    };

    let real_basis = RealBSplineBasis::try_new(basis_config).expect("Failed to construct BSpline Basis");

    let basis_dumper = BasisDump{samples: 10000};

    basis_dumper.dump(&real_basis).expect("Error Dumping BSpline Basis");
}