# peanorust

Small educational Rust project that implements **Peano numbers** (natural numbers built from:

- `Zero`
- `Succ(n)` (the successor of `n`)

This is a classic way to represent natural numbers using only two constructors.

## What’s in here

The core type is:

- `enum Peano { Zero, Succ(Box<Peano>) }`

And a few operations implemented recursively:

- `add(&self, other: &Peano) -> Box<Peano>`: Peano addition
  - `0 + m = m`
  - `S(n) + m = S(n + m)`
- `greater_than(&self, other: &Peano) -> bool`: strict comparison
  - `0 > _` is `false`
  - `S(_) > 0` is `true`
  - `S(n) > S(m)` reduces to `n > m`
- `to_usize(&self) -> usize`: converts the Peano representation into a normal integer for printing

`Box<Peano>` is used so the recursive `Succ` variant has a known size.

## Run it

```bash
cargo run
```

You should see output similar to:

- `Three as usize: 3`
- `one > two: false`

## Notes

- This is intentionally recursive and not optimized; it’s meant as a learning example.
- Built with the Rust 2024 edition (see `Cargo.toml`).

