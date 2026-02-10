use std::fmt;

use crate::q::Rat;

// Reals (R) are deeper than "fractions"; a full construction would be things like
// Dedekind cuts or Cauchy sequences. For this educational tower, we implement a
// rational subset of R and keep the stage boundary explicit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Real(pub Rat);

impl Real {
    pub fn from_rat(r: Rat) -> Real {
        Real(r)
    }

    pub fn add(&self, other: &Real) -> Real {
        Real(self.0.add(&other.0))
    }

    pub fn sub(&self, other: &Real) -> Real {
        Real(self.0.sub(&other.0))
    }

    pub fn mul(&self, other: &Real) -> Real {
        Real(self.0.mul(&other.0))
    }

    pub fn div(&self, other: &Real) -> Option<Real> {
        Some(Real(self.0.div(&other.0)?))
    }
}

impl fmt::Display for Real {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
