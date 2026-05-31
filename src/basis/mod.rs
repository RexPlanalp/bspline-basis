// Internal Submodules
mod complex;
mod evaluator;
mod integrator;
mod real;

// Public API
pub use complex::ComplexBSplineBasis;
pub use evaluator::BSplineBasisEvaluator;
pub use integrator::BSplineBasisIntegrator;
pub use real::RealBSplineBasis;

// Internal Import
use crate::basis::integrator::find_params;
use crate::config::{Config, ConfigError, ConfigResult};
use crate::knot::KnotVector;
use crate::scalar::BSplineScalar;

// External Imports
use ndarray::linspace;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct BasisConfig {
    pub n_basis: usize,
    pub order: usize,
}

impl Config for BasisConfig {
    fn validate(&self) -> ConfigResult<()> {
        if self.order > 8 || self.order < 2 {
            return Err(ConfigError::InvalidOrder { order: self.order });
        }

        Ok(())
    }
}

pub struct BSplineBasis<KV: KnotVector> {
    knot_vector: KV,
    n_basis: usize,
    order: usize,
}

impl<KV: KnotVector> BSplineBasis<KV> {
    pub fn knot_vector(&self) -> &KV {
        &self.knot_vector
    }

    pub fn n_basis(&self) -> usize {
        self.n_basis
    }

    pub fn order(&self) -> usize {
        self.order
    }

    pub fn degree(&self) -> usize {
        self.order - 1
    }

    pub fn evaluator(&self) -> BSplineBasisEvaluator<'_, KV> {
        BSplineBasisEvaluator { basis: self }
    }

    pub fn integrator(&self) -> BSplineBasisIntegrator<'_, KV> {
        let (roots, weights) = find_params(self.order);
        BSplineBasisIntegrator {
            basis: self,
            roots,
            weights,
        }
    }

    pub fn dump(&self, samples: usize) -> std::io::Result<()> {
        self.knot_vector().dump()?;
        std::fs::create_dir_all("output")?;

        let b_output_file = File::create("output/B.txt")?;
        let db_output_file = File::create("output/dB.txt")?;
        let mut b_writer = BufWriter::new(b_output_file);
        let mut db_writer = BufWriter::new(db_output_file);

        let (start, end) = self.knot_vector().parameter_domain();
        let x_range: Vec<f64> = linspace(start, end, samples).collect();

        for i in 0..self.n_basis() {
            for x in &x_range {
                let val = self.evaluator().b(i, *x);
                writeln!(b_writer, "{} {}", val.real(), val.imag())?;

                let val = self.evaluator().db(i, *x);
                writeln!(db_writer, "{} {}", val.real(), val.imag())?;
            }
        }

        let metadata_file = File::create("output/basis_meta.txt")?;
        let mut metadata_writer = BufWriter::new(metadata_file);

        writeln!(metadata_writer, "{}", self.n_basis())?;
        for x in &x_range {
            writeln!(metadata_writer, "{x}")?;
        }

        Ok(())
    }
}
