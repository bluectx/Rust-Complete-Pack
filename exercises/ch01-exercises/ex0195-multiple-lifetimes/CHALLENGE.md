# Multiple lifetimes

**Level:** ⭐ BEGINNER

**Objective:** Gérer plusieurs paramètres de lifetime

**Problem:**

Écrivez une fonction avec 'a et 'b différentes.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Utile quand les références n'ont pas la même durée.
- `fn func<'a, 'b>(x: &'a T, y: &'b T)` permet des lifetimes indépendants.
