#[derive(Clone, Debug)]
enum Peano {
    Zero,
    Succ(Box<Peano>),
}

impl Peano {
    fn add(&self, other: &Peano) -> Box<Peano> {
        match self {
            Peano::Zero => Box::new(other.clone()),
            Peano::Succ(n) => Box::new(Peano::Succ(n.add(other))),
        }
    }

    fn greater_than(&self, other: &Peano) -> bool {
        match (self, other) {
            (Peano::Zero, Peano::Zero) => false,
            (Peano::Zero, _) => false,
            (_, Peano::Zero) => true,
            (Peano::Succ(n), Peano::Succ(m)) => n.greater_than(m),
        }
    }

    fn to_usize(&self) -> usize {
        match self {
            Peano::Zero => 0,
            Peano::Succ(n) => 1 + n.to_usize(),
        }
    }
}

fn main() {
    let _zero = Peano::Zero;
    let one = Peano::Succ(Box::new(Peano::Zero));
    let two = Peano::Succ(Box::new(Peano::Succ(Box::new(Peano::Zero))));

    let three = one.add(&two);
    println!("Three as usize: {}", three.to_usize());

    // try  greater_than
    println!("one > two: {}", one.greater_than(&two));
}
