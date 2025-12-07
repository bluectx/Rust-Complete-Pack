# Move closures dans les threads

**Level:** ⭐⭐ INTERMEDIATE

**Objective:** Capturer des variables dans une closure pour un thread

**Problem:**

Utilisez `move` pour capturer les variables dans la closure du thread.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Nécessaire pour que les threads possèdent leurs données.
- `move` force le transfer d'ownership.
