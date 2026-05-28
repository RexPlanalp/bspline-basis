// Internal Submodules
mod complex;
mod evaluator;
mod real;
mod integrator;

// Public API
pub use complex::ComplexBSplineBasis;
pub use evaluator::BSplineBasisEvaluator;
pub use integrator::BSplineBasisIntegrator;
pub use real::RealBSplineBasis;

use crate::basis::integrator::find_params;
// Internal Import
use crate::config::{Config, ConfigError, ConfigResult};
use crate::knot::KnotVector;

pub struct BasisConfig {
    pub n_basis: usize,
    pub order: usize,
}

impl Config for BasisConfig {
    fn validate(&self) -> ConfigResult<()> {
        if self.order > 8 {
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

    pub fn integrator(&self) -> BSplineBasisIntegrator<'_, KV> {
        let (roots,weights) = find_params(self.order);
        BSplineBasisIntegrator { basis: self, roots, weights }
    }
}
