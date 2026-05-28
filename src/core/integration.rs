// Internal Imports
use crate::knot::KnotVector;
use crate::scalar::BSplineScalar;
use crate::basis::BSplineBasis;

// External Imports
use num_traits::{Zero};
use std::cmp::{min, max};

pub(crate) fn integrate<KV: KnotVector, F>(i: usize, j: usize, basis: &BSplineBasis<KV>, roots: &[f64], weights: &[f64], integrand: F) -> KV::Scalar
where 
    F: Fn(usize, usize, KV::Scalar, &KV, usize) -> KV::Scalar
{
    let mut result = KV::Scalar::zero();

    let half = KV::Scalar::from_f64(0.5);

    let lower = min(i,j);
    let upper = max(i,j);

    for k in lower..=upper + basis.n_basis() {
        let (a,b) = basis.knot_vector().interval(k);

        if a == b {
            continue;
        }

        let half_b_minus_a = half * (b - a);
        let half_b_plus_a = half * (b + a);

        for (r,w) in roots.iter().zip(weights.iter()) {
            let r = KV::Scalar::from_f64(*r);
            let w = KV::Scalar::from_f64(*w) * half_b_minus_a;

            let x = half_b_minus_a * r + half_b_plus_a;

            result = result + w * integrand(i, j, x, &basis.knot_vector(), basis.degree());
        }
    }

    result
}