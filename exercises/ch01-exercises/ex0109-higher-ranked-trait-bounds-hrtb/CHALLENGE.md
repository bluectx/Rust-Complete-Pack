# Higher-ranked trait bounds (HRTB)

**Level:** ⭐ BEGINNER

**Objective:** Utiliser for<'a> pour les trait bounds

**Problem:**

Écrivez un trait bound avec `for<'a>` pour les closures.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Utile pour les callbacks polymorphes.
- `for<'a> Fn(&'a T) -> U` s'applique à tout lifetime.
