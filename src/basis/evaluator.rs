use super::BSplineBasis;
use crate::knot::{ComplexKnotVector, KnotVector, RealKnotVector};
pub struct BSplineBasisEvaluator<'a, KV: KnotVector> {
    pub basis: &'a BSplineBasis<KV>,
}

impl<'a> BSplineBasisEvaluator<'a, RealKnotVector> {
    pub fn b(&self, i: usize, x: f64) -> <RealKnotVector as KnotVector>::Scalar {
        crate::core::evaluation::b(i, x, self.basis.knot_vector(), self.basis.degree())
    }

    pub fn db(&self, i: usize, x: f64) -> <RealKnotVector as KnotVector>::Scalar {
        crate::core::evaluation::db(i, x, self.basis.knot_vector(), self.basis.degree())
    }
}

impl<'a> BSplineBasisEvaluator<'a, ComplexKnotVector> {
    pub fn b(&self, i: usize, x: f64) -> <ComplexKnotVector as KnotVector>::Scalar {
        crate::core::evaluation::b(
            i,
            crate::core::ecs::ecs_x(
                x,
                self.basis.knot_vector.config.r0,
                self.basis.knot_vector.config.eta,
            ),
            self.basis.knot_vector(),
            self.basis.degree(),
        )
    }

    pub fn db(&self, i: usize, x: f64) -> <ComplexKnotVector as KnotVector>::Scalar {
        crate::core::evaluation::db(
            i,
            crate::core::ecs::ecs_x(
                x,
                self.basis.knot_vector.config.r0,
                self.basis.knot_vector.config.eta,
            ),
            self.basis.knot_vector(),
            self.basis.degree(),
        )
    }
}
