// Internal Imports
use crate::config::Config;
use crate::{ BasisConfig, BSplineBasis, ConfigResult, RealKnotConfig, RealKnotVector};

// External Imports

pub type RealBSplineBasis = BSplineBasis<RealKnotVector>;

impl RealBSplineBasis {
    pub fn build(config: &BasisConfig, start: f64, end: f64) -> ConfigResult<Self> {
        config.validate()?;

        let knot_config = RealKnotConfig {
            n_knots: config.n_basis + config.order,
            multiplicity: config.order - 1,
            start,
            end,
        };

        let knot_vector = RealKnotVector::build(&knot_config)?;

        Ok(Self::new(
            knot_vector,
            config.n_basis,
            config.order,
        ))
    }
}