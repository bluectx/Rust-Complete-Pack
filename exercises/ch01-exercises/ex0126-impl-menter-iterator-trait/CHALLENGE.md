# Implémenter Iterator trait

**Level:** ⭐ BEGINNER

**Objective:** Créer un itérateur custom

**Problem:**

Implémentez `impl Iterator for MyStruct { type Item = T; fn next() { } }`.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Les iterators sont au cœur de Rust.
- Iterator trait = next() qui retourne Option<Item>.
