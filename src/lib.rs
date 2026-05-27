mod scalar;
pub use scalar::BSplineScalar;

mod knot;
pub use knot::{KnotVector, RealKnotConfig, RealKnotVector, ComplexKnotConfig, ComplexKnotVector};

mod config;
pub use config::{ConfigError, ConfigResult};

mod core;

mod basis;
pub use basis::{BasisConfig, BSplineBasis, ComplexBSplineBasis};
