use crate::{
    BasisConfig, BSplineBasis, Config, ConfigResult, ComplexKnotConfig, ComplexKnotVector,
};

pub type ComplexBSplineBasis = BSplineBasis<ComplexKnotVector>;

impl ComplexBSplineBasis {
    pub fn build(config: &BasisConfig, start: f64, end: f64, eta: f64, r0: f64) -> ConfigResult<Self> {
        config.validate()?;

        let knot_config = ComplexKnotConfig {
            n_knots: config.n_basis + config.order,
            multiplicity: config.order - 1,
            start,
            end,
            eta,
            r0
        };

        let knot_vector = ComplexKnotVector::build(&knot_config)?;

        Ok(Self::new(
            knot_vector,
            config.n_basis,
            config.order,
        ))
    }
}