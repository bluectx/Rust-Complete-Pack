# Vec - accès par index

**Level:** ⭐ BEGINNER

**Objective:** Accéder aux éléments d'un vecteur

**Problem:**

Créez un vec et accédez à des éléments par index. Essayez une index hors limites.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Les indices out-of-bounds causent une panique avec [index].
- `v[index]` ou `v.get(index)` (plus sûr).
