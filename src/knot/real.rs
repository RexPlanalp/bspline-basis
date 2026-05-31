// Internal Imports
use crate::config::{Config, ConfigError, ConfigResult};
use crate::knot::KnotVector;
use crate::knot::builders::build_linear_knots;
// External Imports

#[derive(Clone)]
pub struct RealKnotConfig {
    pub n_knots: usize,
    pub multiplicity: usize,
    pub start: f64,
    pub end: f64,
}

impl Config for RealKnotConfig {
    fn validate(&self) -> ConfigResult<()> {
        if self.n_knots < 2 * self.multiplicity {
            return Err(ConfigError::InvalidNumberOfKnots {
                n_knots: self.n_knots,
                multiplicity: self.multiplicity,
            });
        }

        if self.start >= self.end {
            return Err(ConfigError::InvalidKnotRange {
                start: self.start,
                end: self.end,
            });
        }

        Ok(())
    }
}

pub struct RealKnotVector {
    config: RealKnotConfig,
    knots: Vec<f64>,
}

impl RealKnotVector {
    pub fn build(config: &RealKnotConfig) -> ConfigResult<Self> {
        config.validate()?;

        let knots = build_linear_knots(&config);

        Ok(Self {
            config: config.clone(),
            knots,
        })
    }
}

impl KnotVector for RealKnotVector {
    type Scalar = f64;
    type Configuration = RealKnotConfig;

    fn config(&self) -> &Self::Configuration {
        &self.config
    }

    fn parameter_domain(&self) -> (f64, f64) {
        (self.config().start, self.config().end)
    }

    fn parameter_to_knot(&self, x: f64) -> Self::Scalar {
        x
    }

    fn knots(&self) -> &[Self::Scalar] {
        &self.knots
    }
}
