use crate::knots::{KnotVector, KnotVectorConfig};

pub struct BSpline {
    n: usize,
    order: usize,
    degree: usize,
    knot_vector: KnotVector
}

impl BSpline {
    pub fn new(start: f64, end: f64, n: usize, order: usize, r0: f64, eta: f64) -> Self {
        let knot_vector_config = KnotVectorConfig {
            start,
            end,
            n: n + order,
            r0,
            eta,
            multiplicity: order -1
        };

        let knot_vector = KnotVector::new(knot_vector_config);

        Self {n, order, degree: order -1, knot_vector}
    }
}