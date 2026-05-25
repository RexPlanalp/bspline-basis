mod real;

use crate::KnotVector;
pub trait BSplineBasis {
    type KV: KnotVector;

    fn knot_vector(&self) -> &Self::KV;
    fn n_basis(&self) -> usize;
    fn order(&self) -> usize;
    fn degree(&self) -> usize;
}
