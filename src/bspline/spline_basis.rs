use crate::knots::knot_vec::KnotVec;
use nalgebra::{DVector, RealField, Vector};

/// A B-spline basis of `n` basis functions of degree `p`.
#[derive(Debug, Clone)]
pub struct SplineBasis<T : RealField> {
    
    /// Knot vector for the allocation of the basis functions.
    pub knots: KnotVec<T>,
    
    /// Number of basis functions.
    pub n: usize,
    
    /// Degree of basis functions.
    pub p: usize,
}

impl<T : RealField + Copy> SplineBasis<T> {

    /// Constructs a new [`SplineBasis`].
    pub fn new(knots: KnotVec<T>, n: usize, p: usize) -> Self {
        SplineBasis { knots, n, p }
    }
    
    /// Constructs a new [`SplineBasis`] on an open knot vector of size `n+p+1`.
    pub fn open(n: usize, p: usize) -> Self {
        Self::new(KnotVec::open(n, p), n, p)
    }
}

impl<T : RealField + Copy> SplineBasis<T> {
    
    /// Finds the index `i` such that `knots[i] <= t < knots[i+1]`.
    pub fn find_span(&self, t: T) -> Result<usize, ()> {
        if t < self.knots.first() || t > self.knots.last() { return Err(()) }
        
        if t == self.knots[self.n + 1] {
            return Ok(self.n - 1);
        }

        let idx = self.knots.0.binary_search_by(|xi| xi.partial_cmp(&t).unwrap());
        match idx {
            Ok(i) => { Ok(i) }
            Err(i) => { Ok(i - 1) }
        }
    }

    /// Evaluates the `p+1` non-vanishing basis functions at the parametric point `t`.
    pub fn eval(&self, t: T) -> DVector<T> {
        let idx = self.find_span(t).unwrap();
        let mut left = vec![T::zero(); self.p + 1];
        let mut right = vec![T::zero(); self.p + 1];
        let mut B = DVector::zeros(self.p + 1);
        B[0] = T::one();

        for i in 1..=self.p {
            left[i] = t - self.knots[idx - i + 1];
            right[i] = self.knots[idx + i] - t;
            let mut saved = T::zero();

            for j in 0..i {
                let tmp = B[j] / (right[j+1] + left[i-j]);
                B[j] = saved + right[j+1]*tmp;
                saved = left[i-j]*tmp;
            }
            B[i] = saved;
        }
        B
    }
}