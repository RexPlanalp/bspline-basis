// Internal imports
use super::BSplineBasis;
use crate::knot::KnotVector;
use crate::scalar::BSplineScalar;

// External imports
use ndarray::linspace;
use num_traits::Zero;

pub fn find_params(order: usize) -> (&'static [f64], &'static [f64]) {
    match order {
        2 => (&[-0.57735027, 0.57735027], &[1.0, 1.0]),
        3 => (
            &[-0.77459667, 0.0, 0.77459667],
            &[0.55555556, 0.88888889, 0.55555556],
        ),
        4 => (
            &[-0.86113631, -0.33998104, 0.33998104, 0.86113631],
            &[0.34785485, 0.65214515, 0.65214515, 0.34785485],
        ),
        5 => (
            &[-0.90617985, -0.53846931, 0.0, 0.53846931, 0.90617985],
            &[0.23692689, 0.47862867, 0.56888889, 0.47862867, 0.23692689],
        ),
        6 => (
            &[
                -0.93246951,
                -0.66120939,
                -0.23861919,
                0.23861919,
                0.66120939,
                0.93246951,
            ],
            &[
                0.17132449, 0.36076157, 0.46791393, 0.46791393, 0.36076157, 0.17132449,
            ],
        ),
        7 => (
            &[
                -0.94910791,
                -0.74153119,
                -0.40584515,
                0.0,
                0.40584515,
                0.74153119,
                0.94910791,
            ],
            &[
                0.12948497, 0.27970539, 0.38183005, 0.41795918, 0.38183005, 0.27970539, 0.12948497,
            ],
        ),
        8 => (
            &[
                -0.96028986,
                -0.79666648,
                -0.52553241,
                -0.18343464,
                0.18343464,
                0.52553241,
                0.79666648,
                0.96028986,
            ],
            &[
                0.10122854, 0.22238103, 0.31370665, 0.36268378, 0.36268378, 0.31370665, 0.22238103,
                0.10122854,
            ],
        ),
        _ => panic!("Order not supported for BSplineBasisIntegrator"),
    }
}
pub struct BSplineBasisIntegrator<'a, KV: KnotVector> {
    pub basis: &'a BSplineBasis<KV>,
    pub roots: &'static [f64],
    pub weights: &'static [f64],
}

impl<'a, KV: KnotVector> BSplineBasisIntegrator<'a, KV> {
    pub fn integrate<F>(&self, i: usize, j: usize, integrand: F) -> KV::Scalar
    where
        F: Fn(usize, usize, KV::Scalar, &KV, usize) -> KV::Scalar,
    {
        crate::core::integration::integrate(i, j, self.basis, self.roots, self.weights, integrand)
    }

    pub fn integrate_kinetic(&self, i: usize, j: usize) -> KV::Scalar {
        crate::core::integration::integrate(
            i,
            j,
            self.basis,
            self.roots,
            self.weights,
            kinetic_integrand,
        )
    }

    pub fn integrate_potential<F>(&self, i: usize, j: usize, potential: F) -> KV::Scalar
    where
        F: Fn(KV::Scalar) -> KV::Scalar,
    {
        self.integrate(i, j, |i, j, x, knot_vector, degree| {
            potential_integrand(i, j, x, knot_vector, degree, &potential)
        })
    }

    pub fn project<F: Fn(KV::Scalar) -> KV::Scalar>(&self, function: F) -> Vec<KV::Scalar> {
        let mut result = Vec::with_capacity(self.basis.n_basis());

        for i in 0..self.basis.n_basis() {
            let val = self.integrate(i, i, |i, _j, x, knot_vector, degree| {
                projection_integrand(i, x, knot_vector, degree, &function)
            });

            result.push(val);
        }

        result
    }

    pub fn expand(&self, projection: &[KV::Scalar], samples: usize) -> Vec<KV::Scalar> {
        assert_eq!(projection.len(), self.basis.n_basis());

        let (start, end) = self.basis.knot_vector().parameter_domain();
        let x_range: Vec<f64> = linspace(start, end, samples).collect();

        let mut result: Vec<KV::Scalar> = vec![KV::Scalar::zero(); samples];
        for i in 0..self.basis.n_basis() {
            for (idx, x) in x_range.iter().enumerate() {
                if self.basis.knot_vector().in_interval_span(self.basis.knot_vector.parameter_to_knot(*x), i, i + self.basis.order()) {
                    result[idx] = result[idx] + projection[i] *  crate::core::evaluation::b(i, self.basis.knot_vector().parameter_to_knot(*x), self.basis.knot_vector(), self.basis.degree());
                }
            }
        }

        result
    }

    pub fn set_quadrature(&mut self, order: usize) {
        let (roots, weights) = find_params(order);
        self.roots = roots;
        self.weights = weights;
    }
}

fn kinetic_integrand<KV: KnotVector>(
    i: usize,
    j: usize,
    x: KV::Scalar,
    knot_vector: &KV,
    degree: usize,
) -> KV::Scalar {
    KV::Scalar::from_f64(0.5)
        * crate::core::evaluation::db(i, x, knot_vector, degree)
        * crate::core::evaluation::db(j, x, knot_vector, degree)
}

fn potential_integrand<KV: KnotVector, F>(
    i: usize,
    j: usize,
    x: KV::Scalar,
    knot_vector: &KV,
    degree: usize,
    potential: F,
) -> KV::Scalar
where
    F: Fn(KV::Scalar) -> KV::Scalar,
{
    crate::core::evaluation::b(i, x, knot_vector, degree)
        * potential(x)
        * crate::core::evaluation::b(j, x, knot_vector, degree)
}

fn projection_integrand<KV: KnotVector, F>(
    i: usize,
    x: KV::Scalar,
    knot_vector: &KV,
    degree: usize,
    function: F,
) -> KV::Scalar
where
    F: Fn(KV::Scalar) -> KV::Scalar,
{
    crate::core::evaluation::b(i, x, knot_vector, degree) * function(x)
}
