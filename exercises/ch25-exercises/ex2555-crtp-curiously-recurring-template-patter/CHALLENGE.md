# CRTP (Curiously Recurring Template Pattern)

**Level:** ⭐⭐ INTERMEDIATE

**Objective:** Utiliser CRTP pour éviter les virtual calls

**Problem:**

Écrivez `struct Base<T> { ... }` et `impl Base<Derived>`.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Performance tout en maintenant la flexibilité.
- CRTP = statically dispatch sans vtables.
