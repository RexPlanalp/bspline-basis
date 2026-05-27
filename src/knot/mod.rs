// Internal Submodules
mod knot_vector;
mod builders;
mod complex;
mod real;

// Public API
pub use real::{RealKnotConfig, RealKnotVector};
pub use complex::{ComplexKnotConfig, ComplexKnotVector};
pub use knot_vector::{KnotVector};

