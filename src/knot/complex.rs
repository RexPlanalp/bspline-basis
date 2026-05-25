use crate::KnotVector;
use crate::core::{ecs_x, find_best_r0};
use crate::knot::builders::build_linear_knots;
use crate::{Config, ConfigError, ConfigResult, RealKnotConfig};
use num_complex::Complex64;
use std::f64::consts::PI;
#[derive(Clone)]
pub struct ComplexKnotConfig {
    pub n_knots: usize,
    pub multiplicity: usize,
    pub start: f64,
    pub end: f64,
    pub r0: f64,
    pub eta: f64,
}

impl Config for ComplexKnotConfig {
    fn validate(&self) -> ConfigResult<()> {
        if self.eta > PI / 2.0 {
            return Err(ConfigError::InvalidEta { eta: self.eta });
        }
        if self.r0 < self.start || self.r0 > self.end {
            return Err(ConfigError::InvalidR0 {
                start: self.start,
                end: self.end,
                r0: self.r0,
            });
        }
        Ok(())
    }
}
pub struct ComplexKnotVector {
    pub config: ComplexKnotConfig,
    pub knots: Vec<Complex64>,
}

impl ComplexKnotVector {
    pub fn build(config: &ComplexKnotConfig) -> ConfigResult<Self> {
        config.validate()?;

        let real_knots = build_linear_knots(&RealKnotConfig {
            n_knots: config.n_knots,
            multiplicity: config.multiplicity,
            start: config.start,
            end: config.end,
        });

        let mut snapped_config = config.clone();
        snapped_config.r0 = find_best_r0(&real_knots, config.r0);

        let knots: Vec<Complex64> = real_knots
            .iter()
            .map(|x| ecs_x(*x, snapped_config.r0, snapped_config.eta))
            .collect();

        Ok(Self {
            config: snapped_config,
            knots,
        })
    }
}

impl KnotVector for ComplexKnotVector {
    type Scalar = Complex64;

    fn knots(&self) -> &[Self::Scalar] {
        &self.knots
    }
}
