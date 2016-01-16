use std::ops::{Mul, Add};

use rand::{Rand, Rng, Closed01};
use num::{Float, Zero};

use rblas::{Gemm, Matrix};
use rblas::attribute::Transpose;

use ndarray::ArrayBase;
use ndarray::blas::AsBlas;
use ndarray::blas;

use helpers::random01;

pub trait ShapeAsTuple<T> {
    fn shape_as_tuple(&self) -> T;
}

impl<T> ShapeAsTuple<(usize, usize)> for ArrayBase<Vec<T>, (usize, usize)> {
    #[inline]
    fn shape_as_tuple(&self) -> (usize, usize) {
        (self.shape()[0], self.shape()[1])
    }
}

// TODO Dimension type alias
// TODO OwnedArray

pub struct OrthogonalNMFBlas<FloatT> {
    pub hidden: ArrayBase<Vec<FloatT>, (usize, usize)>,
    pub weights: ArrayBase<Vec<FloatT>, (usize, usize)>,

    // these hold temporary results during an iteration.
    // kept in struct to prevent unnecessary memory allocations.
    pub tmp_weights_dividend: ArrayBase<Vec<FloatT>, (usize, usize)>,
    // pub tmp_weights_divisor: ArrayBase<Vec<FloatT>, (usize, usize)>,
    // pub tmp_weights_divisor_reconstruction: ArrayBase<Vec<FloatT>, (usize, usize)>,
    //
    // pub tmp_hidden_dividend: ArrayBase<Vec<FloatT>, (usize, usize)>,
    // pub tmp_hidden_divisor: ArrayBase<Vec<FloatT>, (usize, usize)>,
    // pub tmp_hidden_divisor_tmp: ArrayBase<Vec<FloatT>, (usize, usize)>,
}

impl<FloatT> OrthogonalNMFBlas<FloatT>
    where FloatT: Float + Mul + Zero + Clone + Gemm,
          // ArrayBase<Vec<FloatT>, (usize, usize)>: Matrix,
          // ndarray::blas::BlasArrayViewMut
          Closed01<FloatT>: Rand
{
    pub fn init_random01<R: Rng>(nhidden: usize, nobserved: usize, nsamples: usize, rng: &mut R) -> OrthogonalNMFBlas<FloatT> {
        let mut hidden = ArrayBase::zeros((nhidden, nobserved));
        for x in hidden.iter_mut() {
            *x = random01(rng);
        }

        let mut weights = ArrayBase::zeros((nsamples, nhidden));
        for x in weights.iter_mut() {
            *x = random01(rng);
        }

        Self::init(hidden, weights)
    }

    // TODO initialize with values from last time step t-1 instead of choosing randomly
    pub fn init(hidden: ArrayBase<Vec<FloatT>, (usize, usize)>, weights: ArrayBase<Vec<FloatT>, (usize, usize)>) -> OrthogonalNMFBlas<FloatT> {
        let weights_shape = weights.shape_as_tuple();
        OrthogonalNMFBlas {
            hidden: hidden,
            weights: weights,
            tmp_weights_dividend: ArrayBase::zeros(weights_shape),
        }
    }

    /// returns the number of observed variables
    #[inline]
    pub fn nobserved(&self) -> usize {
        self.hidden.shape()[1]
    }

    /// returns the number of hidden variables
    #[inline]
    pub fn nhidden(&self) -> usize {
        self.hidden.shape()[0]
    }

    /// returns the number of data points
    #[inline]
    pub fn nsamples(&self) -> usize {
        self.weights.shape()[0]
    }

    // TODO samples_shape ?
}

impl OrthogonalNMFBlas<f64> {
    // TODO how many iterations ?
    // TODO compare this to the seoung solution
    /// it gets better and better with each iteration.
    /// one observed per column.
    /// one sample per row.
    pub fn iterate(&mut self, alpha: f64, samples: &mut ArrayBase<Vec<f64>, (usize, usize)>) {
        assert_eq!(samples.shape_as_tuple(), (self.nsamples(), self.nobserved()));

        Gemm::gemm(
            &1.,
            Transpose::NoTrans, &samples.blas(),
            Transpose::Trans, &self.hidden.blas(),
            &0.,
            &mut self.tmp_weights_dividend.blas());
//
//         // has the same shape as weights
//         let new_weights_dividend = data.clone().mul(&hidden_transposed);
//         // has the same shape as weights
//         let new_weights_divisor = self.weights.clone().mul(&self.hidden).mul(&hidden_transposed);
//
//         // has the same shape as hidden
//         let new_hidden_dividend = weights_transposed.clone().mul(data);
//
//         // gamma is a symetric matrix with diagonal elements equal to zero
//         // and other elements = alpha
//         let gamma_size = self.nhidden();
//         let mut gamma = DMat::from_elem(gamma_size, gamma_size, alpha);
//
//         // set diagonal to zero
//         for i in 0..gamma_size {
//             gamma[(i, i)] = FloatT::zero();
//         }
//
//         // has the same shape as hidden
//         let new_hidden_divisor = weights_transposed.clone()
//             .mul(&self.weights)
//             .mul(&self.hidden)
//             // we add the previous latents
//             // multiplied by alpha except for the diag which is set to zero
//             .add(gamma.mul(&self.hidden));
//
//         // compute new weights
//         // TODO possibly use simd for this
//         for col in 0..self.weights.ncols() {
//             for row in 0..self.weights.nrows() {
//                 let index = (row, col);
//                 // if we have any zero in any of the matrizes
//                 // self.weights or self.hidden then
//                 // new_weights_divisor[index] will be zero
//                 // TODO how to deal with this
//                 let mut divisor = new_weights_divisor[index];
//                 if FloatT::zero() == divisor {
//                     divisor = FloatT::min_positive_value();
//                 }
//                 debug_assert!(FloatT::zero() != divisor);
//                 self.weights[index] =
//                     self.weights[index] *
//                     new_weights_dividend[index] /
//                     divisor;
//                 if FloatT::zero() == self.weights[index] {
//                     self.weights[index] = FloatT::min_positive_value();
//                 }
//                 debug_assert!(FloatT::zero() != self.weights[index]);
//             }
//         }
//
//         // compute new hidden
//         // TODO possibly use simd for this
//         for col in 0..self.hidden.ncols() {
//             for row in 0..self.hidden.nrows() {
//                 let index = (row, col);
//                 let mut divisor = new_hidden_divisor[index];
//                 if FloatT::zero() == divisor {
//                     divisor = FloatT::min_positive_value();
//                 }
//                 debug_assert!(FloatT::zero() != divisor);
//                 self.hidden[index] =
//                     self.hidden[index] *
//                     new_hidden_dividend[index] /
//                     divisor;
//                 if FloatT::zero() == self.hidden[index] {
//                     self.hidden[index] = FloatT::min_positive_value();
//                 }
//                 debug_assert!(FloatT::zero() != self.hidden[index]);
//             }
//         }
    }
}