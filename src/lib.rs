mod scalar;
pub use scalar::BSplineScalar;

mod knot;
pub use knot::KnotVector;
pub use knot::{RealKnotConfig, RealKnotVector};

mod config;
pub use config::{Config, ConfigError, ConfigResult};

mod core;

mod basis;
