use super::BSplineBasis;
use crate::knot::KnotVector;

pub struct BSplineBasisEvaluator<'a, KV: KnotVector> {
    pub basis: &'a BSplineBasis<KV>,
}

impl<'a, KV: KnotVector> BSplineBasisEvaluator<'a, KV> {
    pub fn b(&self, i: usize, x: f64) -> KV::Scalar {
        crate::core::evaluation::b(
            i,
            self.basis.knot_vector().parameter_to_knot(x),
            self.basis.knot_vector(),
            self.basis.degree(),
        )
    }

    pub fn db(&self, i: usize, x: f64) -> KV::Scalar {
        crate::core::evaluation::db(
            i,
            self.basis.knot_vector().parameter_to_knot(x),
            self.basis.knot_vector(),
            self.basis.degree(),
        )
    }
}
