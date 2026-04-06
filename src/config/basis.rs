use crate::{error::BSplineError, error::Result};

use crate::core::config::Config;

pub struct BasisConfig {
    pub n_knots: usize,
    pub order: usize,
    pub start: f64,
    pub end: f64,
}

impl Config for BasisConfig {
    fn validate(&self) -> Result<()> {
        if !(self.n_knots > 0) {
            return Err(BSplineError::InvalidNumberOfBasis {
                n_knots: self.n_knots,
            });
        }
        if !(self.start > self.end) {
            return Err(BSplineError::InvalidBasisRange {
                start: self.start,
                end: self.end,
            });
        }
        if !(self.order > 0 && self.order <= 8) {
            return Err(BSplineError::InvalidOrder { order: self.order });
        }
        Ok(())
    }
}
