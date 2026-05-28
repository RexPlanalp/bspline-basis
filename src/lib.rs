mod scalar;
pub use scalar::BSplineScalar;

mod knot;
pub use knot::{ComplexKnotConfig, ComplexKnotVector, KnotVector, RealKnotConfig, RealKnotVector};

mod config;
pub use config::{ConfigError, ConfigResult};

mod core;

mod basis;
pub use basis::{BSplineBasis, BasisConfig, ComplexBSplineBasis, RealBSplineBasis};
