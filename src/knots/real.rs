use crate::config::Config;
use crate::config::knots::KnotConfig;
use crate::error::Result;
use crate::knots::KnotVector;
use crate::knots::builder::build_linear_knots;

pub struct RealKnotVector {
    pub config: KnotConfig,
    pub knots: Vec<f64>,
}

impl RealKnotVector {
    pub fn try_new(config: KnotConfig) -> Result<Self> {
        config.validate()?;

        let knots = build_linear_knots(&config);

        Ok(Self { config, knots })
    }
}

impl KnotVector for RealKnotVector {
    type Scalar = f64;

    fn knots(&self) -> &[Self::Scalar] {
        &self.knots
    }

    fn start(&self) -> f64 {
        self.config.start
    }

    fn end(&self) -> f64 {
        self.config.end
    }
}
