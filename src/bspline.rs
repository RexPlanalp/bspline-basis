use crate::knots::{KnotVector, KnotVectorConfig};

struct BSpline {
    order: usize,
    knot_vector: KnotVector
}


impl BSpline {
    pub fn new(order: usize, config: KnotVectorConfig) -> Self {
        let knot_vector = KnotVector::new(config);

        Self {order, knot_vector}
    }
}