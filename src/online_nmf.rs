/*!
factorizes a matrix of times and observed values
into ...

use it by repeatedly calling `update` with data
and inspecting `hidden` and `weights`.
*/

use num::{Zero};
use nalgebra::{DMat};

pub struct OnlineNMF<FloatT> {
    /// maps hidden variables (one per row) to observed variables (one per column).
    /// changes on every `update`.
    /// stays constant in size.
    pub hidden: DMat<FloatT>,
    /// maps times (one per row) to hidden variables (one per column).
    /// changes on every `update`.
    /// inspect this.
    /// will grow in size.
    pub weights: DMat<FloatT>,
}

impl<FloatT: Zero + Clone + Copy> OnlineNMF<FloatT> {
    /// returns an online nonnegative matrix factorization
    /// that is supposed to find `nhidden` hidden variables
    /// when repeatedly being `update`d with columns of `nobserved` variables
    pub fn new(nobserved: usize, nhidden: usize) -> OnlineNMF<FloatT> {
        OnlineNMF {
            hidden: DMat::new_zeros(nhidden, nobserved),
            weights: DMat::new_zeros(0, nhidden)
        }
    }

    /// returns the number of observed variables
    #[inline]
    pub fn nobserved(&self) -> usize {
        self.hidden.ncols()
    }

    /// returns the number of hidden variables
    #[inline]
    pub fn nhidden(&self) -> usize {
        self.hidden.nrows()
    }

    /// returns the number of data points
    #[inline]
    pub fn nsamples(&self) -> usize {
        self.weights.nrows()
    }

    pub fn update(&mut self, new_observed_columns: &DMat<FloatT>) {
        assert_eq!(self.nobserved(), new_observed_columns.nrows());

        // do nothing for now
    }
}
