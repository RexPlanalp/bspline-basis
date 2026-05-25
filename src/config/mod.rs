mod error;

pub use error::{ConfigError, ConfigResult};

pub trait Config {
    fn validate(&self) -> ConfigResult<()>;
}
