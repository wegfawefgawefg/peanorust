# peanorust

Educational playground for exploring "number systems" and how you can build a small **number tower** from simple constructions.

It starts with **Peano naturals** (natural numbers built from:

- `Zero`
- `Succ(n)` (the successor of `n`)

and then builds outward to:

- `N` naturals (Peano)
- `Z` integers
- `Q` rationals
- `R` computable reals (interval/Cauchy-style approximation)
- `C` complex numbers

## What’s in here

The implementation is split by stage:

- `src/n.rs` (`N`)
- `src/z.rs` (`Z`)
- `src/q.rs` (`Q`)
- `src/r.rs` (`R`)
- `src/c.rs` (`C`)

## Run it

```bash
cargo run
```

The output is a pile of small demos for each stage.

## Docs

- `docs/00-what-we-have.md`: what is implemented right now
- `docs/01-what-could-be-next.md`: common "next" number systems/structures after `C`
- `docs/02-searching-the-space.md`: how you could model/search the space of constructions and properties

## Notes

- This is intentionally recursive and not optimized; it’s meant as a learning example.
- Built with the Rust 2024 edition (see `Cargo.toml`).
