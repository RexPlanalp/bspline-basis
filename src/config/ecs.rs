use std::f64::consts::PI;

use crate::{error::BSplineError, error::Result};

use crate::core::config::Config;
pub struct EcsConfig {
    pub r0: f64,
    pub eta: f64,
}

impl Config for EcsConfig {
    fn validate(&self) -> Result<()> {
        if !(self.eta < PI / 2.0) {
            return Err(BSplineError::InvalidEta { eta: self.eta });
        }

        Ok(())
    }
}
