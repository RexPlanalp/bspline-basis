// Internal Imports

// External Imports
use std::fmt;

pub type ConfigResult<T> = std::result::Result<T, ConfigError>;

#[derive(Debug, Clone)]
pub enum ConfigError {
    InvalidNumberOfKnots { n_knots: usize, multiplicity: usize },
    InvalidKnotRange { start: f64, end: f64 },
    InvalidEta { eta: f64 },
    InvalidR0 { start: f64, end: f64, r0: f64 },
    InvalidOrder { order: usize },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidNumberOfKnots {
                n_knots,
                multiplicity,
            } => {
                let twice_multiplicity = 2 * multiplicity;
                write!(
                    f,
                    "n_knots: {n_knots} should be greater than or equal to 2 * multiplicity: {twice_multiplicity}."
                )
            }
            ConfigError::InvalidKnotRange { start, end } => {
                write!(f, "Start: {start} should be less than end: {end}.")
            }
            ConfigError::InvalidEta { eta } => {
                write!(f, "Eta: {eta} must be between 0.0 and PI / 2.0.")
            }
            ConfigError::InvalidR0 { start, end, r0 } => {
                write!(f, "R0: {r0} must be between start: {start} and end: {end}.")
            }
            ConfigError::InvalidOrder { order } => {
                write!(f, "Order: {order} not in supported range.")
            }
        }
    }
}
