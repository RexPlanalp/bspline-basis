use crate::config::Config;
use crate::config::ecs::EcsConfig;
use crate::config::knots::KnotConfig;
use crate::error::Result;
use crate::knots::KnotVector;
use crate::knots::builder::build_linear_knots;
use crate::transform::ecs::{ecs_x, find_best_r0};
use num_complex::Complex64;

pub struct ComplexKnotVector {
    pub config: KnotConfig,
    pub ecs_config: EcsConfig,
    pub knots: Vec<Complex64>,
}

impl ComplexKnotVector {
    pub fn try_new(config: KnotConfig, mut ecs_config: EcsConfig) -> Result<Self> {
        config.validate()?;
        ecs_config.validate()?;

        let real_knots = build_linear_knots(&config);

        ecs_config.r0 = find_best_r0(&real_knots, ecs_config.r0);

        let knots: Vec<Complex64> = real_knots
            .iter()
            .map(|x| ecs_x(*x, ecs_config.r0, ecs_config.eta))
            .collect();

        Ok(Self {
            config,
            ecs_config,
            knots,
        })
    }
}

impl KnotVector for ComplexKnotVector {
    type Scalar = Complex64;

    fn knots(&self) -> &[Self::Scalar] {
        &self.knots
    }

    fn start(&self) -> f64 {
        self.config.start
    }

    fn end(&self) -> f64 {
        self.config.end
    }
}
