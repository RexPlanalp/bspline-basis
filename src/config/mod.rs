// Internal Submodules
mod error;

// Public API
pub use error::{ConfigError, ConfigResult};

pub trait Config {
    fn validate(&self) -> ConfigResult<()>;
}
