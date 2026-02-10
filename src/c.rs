use std::fmt;

use crate::r::Real;

// Complex numbers (C) built on top of our "Real" stage.
#[derive(Clone, Debug)]
pub struct Complex {
    pub re: Real,
    pub im: Real,
}

impl Complex {
    pub fn new(re: Real, im: Real) -> Complex {
        Complex { re, im }
    }

    pub fn add(&self, other: &Complex) -> Complex {
        Complex {
            re: self.re.add(&other.re),
            im: self.im.add(&other.im),
        }
    }

    pub fn sub(&self, other: &Complex) -> Complex {
        Complex {
            re: self.re.sub(&other.re),
            im: self.im.sub(&other.im),
        }
    }

    pub fn mul(&self, other: &Complex) -> Complex {
        // (a+bi)(c+di) = (ac - bd) + (ad + bc)i
        let ac = self.re.mul(&other.re);
        let bd = self.im.mul(&other.im);
        let ad = self.re.mul(&other.im);
        let bc = self.im.mul(&other.re);
        Complex {
            re: ac.sub(&bd),
            im: ad.add(&bc),
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Keep it simple and readable for demo output.
        write!(f, "({} + {}i)", self.re, self.im)
    }
}
