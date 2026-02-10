# What Could Be Next

After `C` there is no single universally-agreed "next rung" the way `N ⊂ Z ⊂ Q ⊂ R ⊂ C` feels canonical.
Instead, you pick a direction depending on which algebraic properties you want.

This file is a grab bag of common "next" structures and what they buy/break.

## Hypercomplex Directions

### Quaternions `H`

- 4D extension of complex numbers.
- Multiplication is associative but not commutative.
- Still a division algebra: nonzero elements have inverses.
- Very common in 3D rotations (usually implemented over floats).

### Octonions `O`

- 8D extension.
- Not associative (still alternative).
- Interesting mathematically, less common in typical software.

## "Collections" With Structure

### Vectors `R^n`, `C^n`

- Vector space.
- Has addition and scalar multiplication.
- No general notion of division.

### Matrices `Mat(n, R)` / `Mat(n, C)`

- Ring (and an algebra over `R`/`C`).
- Not commutative for `n > 1`.
- "Division" becomes "multiply by inverse" when invertible.
- Extremely common and practical.

## Algebraic Constructions

### Polynomials `F[x]` (where `F` is a field, e.g. `Q` or `R`)

- A commutative ring.
- Great for algebra and for building more things from.

### Field of Fractions / Rational Functions `F(x)`

- The field of fractions of the polynomial ring `F[x]`.
- Elements look like `p(x)/q(x)` where `q(x) != 0`.

### Adjoining Roots / Algebraic Numbers

- Extend a base field by adding an element satisfying a polynomial equation.
- Example idea: `Q(√2)` or "all algebraic numbers".
- This is a route toward exact representations of many irrationals (but not transcendentals like pi/e).

## Other Useful "Number-Like" Systems

### Dual Numbers `R[ε]/(ε^2)`

- Elements look like `a + bε` with `ε^2 = 0`.
- Used for automatic differentiation.
- Not a field (nonzero non-invertible elements exist).

### Split-Complex / Hyperbolic Numbers `R[j]/(j^2 - 1)`

- Like complex numbers but `j^2 = +1`.
- Has zero divisors; not a field.

### Tropical Semiring

- Replace `+` and `*` with `(min, +)` or `(max, +)`.
- Used in shortest paths, optimization, dynamic programming.

## Suggestion For This Repo

If we want a strong "next stage" that still feels like arithmetic:

- Add quaternions `H` over our `Real` stage.
- Or add polynomial rings over `Q`, then build rational functions as a field of fractions.

