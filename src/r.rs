use std::cmp::Ordering;
use std::fmt;
use std::sync::Arc;

use crate::n::Nat;
use crate::q::Rat;
use crate::z::Int;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Interval {
    pub lo: Rat,
    pub hi: Rat,
}

impl Interval {
    pub fn new(lo: Rat, hi: Rat) -> Interval {
        assert!(lo <= hi, "invalid interval: lo > hi");
        Interval { lo, hi }
    }

    pub fn point(x: Rat) -> Interval {
        Interval {
            lo: x.clone(),
            hi: x,
        }
    }

    pub fn width(&self) -> Rat {
        self.hi.sub(&self.lo)
    }

    pub fn contains_zero(&self) -> bool {
        let z = Rat::zero();
        self.lo <= z && z <= self.hi
    }
}

// Computable/Cauchy-style reals via interval approximation.
//
// Contract: `interval(k)` returns [lo, hi] with:
// - lo <= x <= hi
// - width(hi - lo) <= 1/k   (k >= 1)
//
// We use 1/k (not 2^-k) to keep the Peano denominators manageable.
#[derive(Clone)]
pub struct Real {
    interval_fn: Arc<dyn Fn(usize) -> Interval + Send + Sync>,
}

impl Real {
    pub fn from_rat(r: Rat) -> Real {
        Real {
            interval_fn: Arc::new(move |_k| Interval::point(r.clone())),
        }
    }

    pub fn interval(&self, k: usize) -> Interval {
        assert!(k >= 1, "k must be >= 1");
        (self.interval_fn)(k)
    }

    pub fn add(&self, other: &Real) -> Real {
        let a = self.clone();
        let b = other.clone();
        Real {
            interval_fn: Arc::new(move |k| {
                // If each width <= 1/(2k), then sum width <= 1/k.
                let kk = k.saturating_mul(2).max(1);
                let ia = a.interval(kk);
                let ib = b.interval(kk);
                Interval::new(ia.lo.add(&ib.lo), ia.hi.add(&ib.hi))
            }),
        }
    }

    pub fn sub(&self, other: &Real) -> Real {
        let a = self.clone();
        let b = other.clone();
        Real {
            interval_fn: Arc::new(move |k| {
                let kk = k.saturating_mul(2).max(1);
                let ia = a.interval(kk);
                let ib = b.interval(kk);
                Interval::new(ia.lo.sub(&ib.hi), ia.hi.sub(&ib.lo))
            }),
        }
    }

    pub fn mul(&self, other: &Real) -> Real {
        let a = self.clone();
        let b = other.clone();
        Real {
            interval_fn: Arc::new(move |k| {
                // Refine operand intervals until product interval is narrow enough.
                let target = Rat::new(Int::from_nat(Nat::one()), Nat::from_usize(k)).unwrap(); // 1/k
                let mut p = k.max(1);
                loop {
                    let ia = a.interval(p);
                    let ib = b.interval(p);
                    let prod = interval_mul(&ia, &ib);
                    if prod.width() <= target {
                        return prod;
                    }
                    p = p.saturating_mul(2);
                }
            }),
        }
    }

    pub fn div(&self, other: &Real) -> Option<Real> {
        let a = self.clone();
        let b = other.clone();
        Some(Real {
            interval_fn: Arc::new(move |k| {
                let target = Rat::new(Int::from_nat(Nat::one()), Nat::from_usize(k)).unwrap(); // 1/k
                let mut p = k.max(1);
                loop {
                    let ia = a.interval(p);
                    let ib = b.interval(p);
                    if ib.contains_zero() {
                        // For a nonzero real, higher precision will eventually separate it from 0.
                        p = p.saturating_mul(2);
                        continue;
                    }
                    let inv = interval_inv(&ib);
                    let q = interval_mul(&ia, &inv);
                    if q.width() <= target {
                        return q;
                    }
                    p = p.saturating_mul(2);
                }
            }),
        })
    }
}

impl fmt::Debug for Real {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Debug shows a moderately precise interval.
        let i = self.interval(16);
        write!(f, "Real([{}, {}])", i.lo, i.hi)
    }
}

impl fmt::Display for Real {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display a small interval to show the value is approximated.
        let i = self.interval(16);
        if i.lo == i.hi {
            write!(f, "{}", i.lo)
        } else {
            write!(f, "[{}, {}]", i.lo, i.hi)
        }
    }
}

fn interval_mul(a: &Interval, b: &Interval) -> Interval {
    let ac = a.lo.mul(&b.lo);
    let ad = a.lo.mul(&b.hi);
    let bc = a.hi.mul(&b.lo);
    let bd = a.hi.mul(&b.hi);

    let mut lo = ac.clone();
    let mut hi = ac;
    for x in [ad, bc, bd] {
        if x < lo {
            lo = x.clone();
        }
        if x > hi {
            hi = x;
        }
    }
    Interval::new(lo, hi)
}

fn interval_inv(a: &Interval) -> Interval {
    assert!(!a.contains_zero(), "cannot invert interval containing 0");
    // For monotone 1/x on intervals that do not cross 0:
    // - if interval is positive: [1/hi, 1/lo]
    // - if interval is negative: [1/lo, 1/hi] (order flips twice because both negative)
    let one = Rat::one();
    let inv_lo = one.div(&a.lo).expect("lo != 0");
    let inv_hi = one.div(&a.hi).expect("hi != 0");
    match a.lo.cmp(&Rat::zero()) {
        Ordering::Greater => Interval::new(inv_hi, inv_lo),
        Ordering::Less => Interval::new(inv_lo, inv_hi),
        Ordering::Equal => unreachable!("contains_zero would have been true"),
    }
}
