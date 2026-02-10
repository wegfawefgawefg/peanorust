mod c;
mod n;
mod q;
mod r;
mod z;

use crate::c::Complex;
use crate::n::Nat;
use crate::q::Rat;
use crate::r::Real;
use crate::z::Int;

fn main() {
    println!("== N (Peano naturals) ==");
    let n1 = Nat::from_usize(1);
    let n2 = Nat::from_usize(2);
    let n3 = n1.add(&n2);
    println!("1 + 2 = {}", n3);
    println!("2 * 3 = {}", Nat::from_usize(2).mul(&Nat::from_usize(3)));
    println!(
        "7 - 3 = {}",
        Nat::from_usize(7).sub(&Nat::from_usize(3)).unwrap()
    );
    println!("1 - 3 = {:?}", Nat::from_usize(1).sub(&Nat::from_usize(3)));
    let (q, r) = Nat::from_usize(7).div_mod(&Nat::from_usize(3)).unwrap();
    println!("7 / 3 => q={}, r={}", q, r);
    println!(
        "gcd(42, 30) = {}",
        Nat::from_usize(42).gcd(&Nat::from_usize(30))
    );

    println!("\n== Z (Integers) ==");
    let three = Int::from_nat(Nat::from_usize(3));
    let five = Int::from_nat(Nat::from_usize(5));
    let neg_two = three.sub(&five);
    println!("3 - 5 = {}", neg_two);
    println!(
        "(3 - 5) + 7 = {}",
        neg_two.add(&Int::from_nat(Nat::from_usize(7)))
    );
    println!(
        "-3 * 4 = {}",
        Int::from_diff(Nat::zero(), Nat::from_usize(3)).mul(&Int::from_nat(Nat::from_usize(4)))
    );

    println!("\n== Q (Rationals) ==");
    let half = Rat::new(Int::from_nat(Nat::from_usize(2)), Nat::from_usize(4)).unwrap();
    println!("2/4 normalized = {}", half);
    let one_half = Rat::new(Int::from_nat(Nat::one()), Nat::from_usize(2)).unwrap();
    let one_third = Rat::new(Int::from_nat(Nat::one()), Nat::from_usize(3)).unwrap();
    println!("1/2 + 1/3 = {}", one_half.add(&one_third));
    let neg_two_thirds = Rat::new(
        Int::from_diff(Nat::zero(), Nat::from_usize(2)),
        Nat::from_usize(3),
    )
    .unwrap();
    let three_fourths = Rat::new(Int::from_nat(Nat::from_usize(3)), Nat::from_usize(4)).unwrap();
    println!("-2/3 * 3/4 = {}", neg_two_thirds.mul(&three_fourths));
    println!("(1/2) / (1/3) = {}", one_half.div(&one_third).unwrap());

    println!("\n== R (Cauchy/interval reals; rationals are points) ==");
    let r1 = Real::from_rat(one_half.clone());
    let r2 = Real::from_rat(one_third.clone());
    println!("1/2 + 1/3 = {}", r1.add(&r2));

    println!("\n== C (Complex over R) ==");
    let z1 = Complex::new(
        Real::from_rat(one_half.clone()),
        Real::from_rat(one_third.clone()),
    );
    let z2 = Complex::new(
        Real::from_rat(Rat::new(Int::from_nat(Nat::from_usize(1)), Nat::from_usize(6)).unwrap()),
        Real::from_rat(one_third.neg()),
    );
    println!("z1 = {}", z1);
    println!("z2 = {}", z2);
    println!("z1 + z2 = {}", z1.add(&z2));
    println!("z1 * z2 = {}", z1.mul(&z2));
}
