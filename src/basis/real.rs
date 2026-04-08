use crate::config::basis::BasisConfig;
use crate::knots::real::RealKnotVector;
use crate::core::basis::BSplineBasis;
use crate::config::knots::KnotConfig;
use crate::error::Result;

pub struct RealBSplineBasis {
    knot_vector: RealKnotVector,
    config: BasisConfig,
}

impl BSplineBasis for RealBSplineBasis {
    type KV = RealKnotVector;

    fn b(&self, i: usize, x: f64) -> <Self::KV as crate::core::knot_vector::KnotVector>::Scalar {
        crate::core::eval::b(i, x, self.knot_vector(), self.degree())
    }

    fn db(&self, i: usize, x: f64) -> <Self::KV as crate::core::knot_vector::KnotVector>::Scalar {
        crate::core::eval::db(i, x, self.knot_vector(), self.degree())
    }

    fn knot_vector(&self) -> &Self::KV {
        &self.knot_vector
    }

    fn n_basis(&self) -> usize {
        self.config.n_basis
    }

    fn order(&self) -> usize {
        self.config.order
    }

    fn degree(&self) -> usize {
        self.config.order - 1
    }
}

impl RealBSplineBasis {
    pub fn try_new(config: BasisConfig) -> Result<Self> {

        let knot_config = KnotConfig {
            n_knots: config.n_basis + config.order,
            multiplicity: config.order - 1,
            start: config.start,
            end: config.end,
        };

        let knot_vector = RealKnotVector::try_new(knot_config)?;

        Ok(Self {
            knot_vector,
            config
        })
    }
}