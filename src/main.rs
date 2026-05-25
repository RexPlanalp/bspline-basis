use std::f64::consts::PI;
use bspline_basis::{BasisConfig, ComplexBSplineBasis, KnotVector};


use ndarray::linspace;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {

    let config = BasisConfig{
        n_basis: 30,
        order: 7
    };
    let complex_basis = ComplexBSplineBasis::build(&config, 0.0, 10.0, PI / 4.0, 5.0).unwrap();

    complex_basis.knot_vector().dump().unwrap();


    let output_file = File::create("output/B.txt").unwrap();
    let mut writer = BufWriter::new(output_file);

    let x_range: Vec<f64> = linspace(complex_basis.knot_vector().config.start, complex_basis.knot_vector().config.end, 1000).collect();

    for i in 0..30 {
        for &x in &x_range {
            let val = complex_basis.evaluator().b(i, x);
            writeln!(writer, "{} {}", val.re, val.im).unwrap();
        }
    }


    let metadata_file = File::create("output/basis_meta.txt").unwrap();
    let mut writer = BufWriter::new(metadata_file);

    writeln!(writer, "{}", complex_basis.n_basis()).unwrap();
    for &x in &x_range {
        writeln!(writer, "{x}").unwrap();
    }

}
