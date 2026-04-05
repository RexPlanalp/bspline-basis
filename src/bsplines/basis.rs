use crate::knots::knot_vector::KnotVector;

pub trait BSplineBasis {
    type Config;
    type KV: KnotVector;

    fn new(config: Self::Config) -> Self;
    fn b(&self, i: usize, x: f64) -> <Self::KV as KnotVector>::Scalar;
    fn db(&self, i: usize, x: f64) -> <Self::KV as KnotVector>::Scalar;
    fn knot_vector(&self) -> &Self::KV;
    fn n_basis(&self) -> usize;
}
