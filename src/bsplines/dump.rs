use crate::bsplines::basis::BSplineBasis;
use crate::knots::dump::dump_knots;
use crate::knots::knot_vector::KnotVector;
use ndarray::linspace;
use num_complex::ComplexFloat;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn dump_basis<B>(basis: &B, samples: usize) -> std::io::Result<()>
where
    B: BSplineBasis,
{
    dump_knots(basis.knot_vector())?;

    let output_file = File::create("output/B.txt")?;
    let mut writer = BufWriter::new(output_file);

    let x_range: Vec<f64> = linspace(
        basis.knot_vector().start(),
        basis.knot_vector().end(),
        samples,
    )
    .collect();

    for i in 0..basis.n_basis() {
        for &x in &x_range {
            let eval = basis.b(i, x);
            writeln!(writer, "{} {}", eval.re(), eval.im())?;
        }
    }

    let output_file = File::create("output/dB.txt")?;
    let mut writer = BufWriter::new(output_file);

    for i in 0..basis.n_basis() {
        for &x in &x_range {
            let eval = basis.db(i, x);
            writeln!(writer, "{} {}", eval.re(), eval.im())?;
        }
    }

    let metadata_file = File::create("output/basis_meta.txt")?;
    let mut writer = BufWriter::new(metadata_file);

    writeln!(writer, "{}", basis.n_basis())?;
    for &x in &x_range {
        writeln!(writer, "{x}")?;
    }

    Ok(())
}
