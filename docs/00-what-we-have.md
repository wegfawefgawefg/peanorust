# What We Have (Current Tower)

This repo is an educational "number tower" implemented in Rust. Each stage is built on the previous one, keeping the construction explicit (even when it is slow).

## Stages

### `N` (Naturals) - `src/n.rs`

Peano naturals:

- `Nat::Zero`
- `Nat::Succ(Box<Nat>)`

Operations:

- `add`: total
- `sub`: partial (`Option<Nat>`) since `a - b` is not always in `N`
- `mul`: total (repeated addition)
- `div_mod`: Euclidean division by repeated subtraction (returns quotient and remainder)
- `gcd`: Euclid's algorithm built on `div_mod`

Conversions ("escape hatches"):

- `from_usize`, `to_usize` for printing and sanity checks.

### `Z` (Integers) - `src/z.rs`

Integers constructed from naturals, normalized into a canonical form:

- `Int::Neg(Nat)` (magnitude > 0)
- `Int::Zero`
- `Int::Pos(Nat)` (magnitude > 0)

Operations:

- `add`, `sub`, `mul`: total
- `Ord` / `PartialOrd`: implemented so rationals and intervals can be compared.

### `Q` (Rationals) - `src/q.rs`

Rationals are fractions with an integer numerator and a positive natural denominator:

- `Rat { num: Int, den: Nat }` with invariant `den != 0`

Normalization:

- Reduce by `gcd(|num|, den)`
- Keep denominator positive by construction.

Operations:

- `add`, `sub`, `mul`: total
- `div`: partial (`None` on division by zero)
- `Ord` / `PartialOrd`: via cross-multiplication.

### `R` (Reals) - `src/r.rs`

Computable/Cauchy-style reals via interval approximation.

Representation:

- `Real` stores a function `interval(k)` returning an interval `[lo, hi]` of rationals.
- Contract (informal): `lo <= x <= hi` and `width = hi - lo <= 1/k` for `k >= 1`.

Operations:

- `add`, `sub`: combine intervals (ask inputs at higher internal precision).
- `mul`, `div`: refine operand intervals until the output interval is narrow enough.
  - Division keeps refining until the divisor interval no longer contains 0.

Important note:

- Equality for general reals is not decidable from finite approximations, so we do not try to make `Real: Eq`.

### `C` (Complex) - `src/c.rs`

Complex numbers built over `Real`:

- `Complex { re: Real, im: Real }`

Operations:

- `add`, `sub`, `mul` implemented in the usual way.

## Demo Runner

`src/main.rs` prints a set of demo computations across `N`, `Z`, `Q`, `R`, and `C`.

