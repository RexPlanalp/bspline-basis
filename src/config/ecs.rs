use std::f64::consts::PI;

use crate::{
    config::{Config, Result},
    error::BSplineError,
};
pub struct EcsConfig {
    r0: f64,
    eta: f64
}

impl Config for EcsConfig {
    fn validate(&self) -> Result<()> {
        if !(self.eta < PI / 2.0) {
            return Err(BSplineError::InvalidEta { eta: self.eta })
        }

        Ok(())
    }
}