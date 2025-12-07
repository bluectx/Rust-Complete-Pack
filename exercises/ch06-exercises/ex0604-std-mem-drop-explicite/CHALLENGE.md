# std::mem::drop() explicite

**Level:** ⭐⭐ INTERMEDIATE

**Objective:** Forcer le drop d'une valeur manuellement

**Problem:**

Utilisez `std::mem::drop(value);` pour libérer explicitement.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Rarement nécessaire, mais parfois utile pour le contrôle.
- drop() appelle Drop::drop() immédiatement.
