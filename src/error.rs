use std::fmt::{self};
pub enum BSplineError {
    InvalidNumberOfKnots { n_knots: usize, multiplicity: usize },
    InvalidKnotRange { start: f64, end: f64 },
    InvalidEta { eta: f64 },
    InvalidNumberOfBasis { n_knots: usize },
    InvalidBasisRange { start: f64, end: f64 },
    InvalidOrder { order: usize },
}

pub type Result<T> = std::result::Result<T, BSplineError>;

impl fmt::Display for BSplineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BSplineError::InvalidNumberOfKnots {
                n_knots,
                multiplicity,
            } => {
                write!(
                    f,
                    "n_knots: {} should be greater than or equal to 2 * multiplicity: {}.",
                    n_knots,
                    2 * multiplicity
                )
            }
            BSplineError::InvalidKnotRange { start, end } => {
                write!(f, "Start: {} should be less than end: {}.", start, end)
            }
            BSplineError::InvalidEta { eta } => {
                write!(f, "Eta: {} should be less than 90 degrees.", eta)
            }
            BSplineError::InvalidNumberOfBasis { n_knots } => {
                write!(
                    f,
                    "Number of knots: {} should be greater than or equal to zero.",
                    n_knots
                )
            }
            BSplineError::InvalidOrder { order } => {
                write!(f, "Order: {} should be between 1 and 8.", order)
            }
            BSplineError::InvalidBasisRange { start, end } => {
                write!(f, "Start: {} should be less than end: {}.", start, end)
            }
        }
    }
}
