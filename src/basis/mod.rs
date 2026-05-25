mod real;
mod complex;
pub use complex::ComplexBSplineBasis;
mod evaluator;
pub use evaluator::BSplineBasisEvaluator;
use crate::{Config, ConfigError, KnotVector};

pub struct BasisConfig {
    pub n_basis: usize,
    pub order: usize,
}

impl Config for BasisConfig {
    fn validate(&self) -> crate::ConfigResult<()> {
        if self.order > 8 {
            return Err(ConfigError::InvalidOrder {
                order: self.order,
            });
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
    pub fn new(knot_vector: KV, n_basis: usize, order: usize) -> Self {
        Self {
            knot_vector,
            n_basis,
            order,
        }
    }

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
}
