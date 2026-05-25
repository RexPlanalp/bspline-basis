use crate::{
    Config, ConfigError, ConfigResult, RealKnotConfig, RealKnotVector, basis::BSplineBasis, knot,
};

pub struct RealBasisConfig {
    pub n_basis: usize,
    pub order: usize,
}

impl Config for RealBasisConfig {
    fn validate(&self) -> crate::ConfigResult<()> {
        if self.order > 8 {
            return Err(ConfigError::InvalidOrder { order: self.order });
        }

        Ok(())
    }
}

pub struct RealBsplineBasis {
    knot_vector: RealKnotVector,
    n_basis: usize,
    order: usize,
}

impl RealBsplineBasis {
    pub fn build(config: &RealBasisConfig, start: f64, end: f64) -> ConfigResult<Self> {
        config.validate()?;

        let knot_config = RealKnotConfig {
            n_knots: config.n_basis + config.order,
            multiplicity: config.order - 1,
            start: start,
            end: end,
        };

        let knot_vector = RealKnotVector::build(&knot_config)?;

        Ok(Self {
            knot_vector,
            n_basis: config.n_basis,
            order: config.order,
        })
    }
}

impl BSplineBasis for RealBsplineBasis {
    type KV = RealKnotVector;

    fn knot_vector(&self) -> &Self::KV {
        &self.knot_vector
    }

    fn n_basis(&self) -> usize {
        self.n_basis
    }

    fn order(&self) -> usize {
        self.order
    }

    fn degree(&self) -> usize {
        self.order - 1
    }
}
