# Où clauses (where)

**Level:** ⭐⭐ INTERMEDIATE

**Objective:** Utiliser where pour des trait bounds complexes

**Problem:**

Écrivez `fn f<T>(x: T) where T: Clone + Display { }`.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Préférée pour les bounds complexes.
- where offre une syntaxe plus lisible pour les bounds multiples.
