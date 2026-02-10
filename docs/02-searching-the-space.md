# Searching The Space Of "Number Systems"

The idea: start from `N` and explore outward by applying "construction steps" that produce new algebraic structures.
Then label each structure with properties (ring? field? ordered? commutative?) and see where those properties hold or break.

You can make this precise enough to do graph search (BFS/DFS), but you need:

- a vocabulary of constructions (edges)
- a set of properties (labels)
- rules for how properties transform under each construction

## Two Kinds Of Graphs

### 1) Graph Of Constructions (Types)

Nodes: concrete structures you build.

Examples:

- `N` (Peano naturals)
- `Z` (Grothendieck completion / differences of naturals)
- `Q` (field of fractions of `Z`)
- `R` (completion / computable reals)
- `C` (adjoin `i` with `i^2 + 1 = 0`)
- `Mat(n, R)` (matrix ring)
- `R[x]` (polynomial ring)
- `Frac(R[x])` (rational functions)
- `R[ε]/(ε^2)` (dual numbers)

Edges: named constructors.

Examples:

- `Grothendieck(N) -> Z`
- `Frac(Z) -> Q`
- `Complete(Q) -> R`
- `AdjoinRoot(R, x^2+1) -> C`
- `Matrix(n, A) -> Mat(n, A)`
- `Poly(A) -> A[x]`
- `Quotient(A, ideal) -> A/I`

### 2) Graph Of Properties (Axiom Bundles)

Nodes: sets of axioms (commutative? associative? has 1? no zero divisors?).

Edges: implication relationships ("field implies integral domain", etc.).

This becomes more like a lattice/poset than a free-form graph.

## Minimal Property Set To Track

You can start with a small boolean feature vector:

- additive commutative monoid? (has `0`, `+`, associativity/commutativity)
- additive group? (subtraction always possible)
- multiplicative monoid? (has `1`, `*`, associativity)
- distributive? (ring-like)
- commutative multiplication?
- has no zero divisors? (integral domain)
- every nonzero has inverse? (field / division ring)
- ordered? (total order compatible with +/*)
- complete? (analysis-ish notion; for `R`)

## Rules: Property Propagation Examples

Some rules are simple:

- If `A` is a commutative ring, then `A[x]` is a commutative ring.
- If `A` is a field, `Mat(n, A)` is a ring but not commutative for `n > 1`.
- If `A` is an integral domain, `Frac(A)` is a field.
- `Dual(A)` typically introduces zero divisors (not a domain).

Others are subtle:

- Completeness and order interact strongly with the chosen construction of `R`.
- Equality/ordering may be undecidable for general computable reals.

## A Practical Search Strategy For This Repo

To make this a project and not an infinite swamp:

1. Define a small grammar of constructions we care about.
2. Hard-code a small set of base structures (`N`, `Z`, `Q`, `R`, `C`).
3. Encode property propagation rules (heuristic is fine for education).
4. BFS outward up to a fixed depth (say 3 or 4).
5. Emit a "map" report: each node plus its properties, and where properties break.

## What This Is "Called"

Depending on how formal you want to get, this touches:

- universal algebra
- category theory (constructions as functors)
- typeclass hierarchies / algebraic traits (programming side)
- automated reasoning / finite model finding (checking whether axioms imply properties)

