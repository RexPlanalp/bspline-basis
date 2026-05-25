use crate::{BSplineScalar, KnotVector};
use num_traits::{One, Zero};

pub fn b<KV>(i: usize, x: KV::Scalar, knot_vector: &KV, degree: usize) -> KV::Scalar
where
    KV: KnotVector,
{
    if degree == 0 {
        return if knot_vector.in_interval(x, i) {
            KV::Scalar::one()
        } else {
            KV::Scalar::zero()
        };
    }

    let knots = knot_vector.knots();

    let denom1 = knots[i + degree] - knots[i];
    let denom2 = knots[i + degree + 1] - knots[i + 1];

    let term1 = if denom1.abs_f64() != 0.0 {
        (x - knots[i]) / denom1 * b(i, x, knot_vector, degree - 1)
    } else {
        KV::Scalar::zero()
    };

    let term2 = if denom2.abs_f64() != 0.0 {
        (knots[i + degree + 1] - x) / denom2 * b(i + 1, x, knot_vector, degree - 1)
    } else {
        KV::Scalar::zero()
    };

    term1 + term2
}


pub fn db<KV>(i: usize, x: KV::Scalar, knot_vector: &KV, degree: usize) -> KV::Scalar
where
    KV: KnotVector,
{
    if degree == 0 {
        return KV::Scalar::zero();
    }

    let knots = knot_vector.knots();

    let denom1 = knots[i + degree] - knots[i];
    let denom2 = knots[i + degree + 1] - knots[i + 1];

    let term1 = if denom1.abs_f64() != 0.0 {
        KV::Scalar::from_usize(degree) / denom1 * b(i, x, knot_vector, degree - 1)
    } else {
        KV::Scalar::zero()
    };

    let term2 = if denom2.abs_f64() != 0.0 {
        KV::Scalar::from_usize(degree) / denom2 * b(i + 1, x, knot_vector, degree - 1)
    } else {
        KV::Scalar::zero()
    };

    term1 - term2
}