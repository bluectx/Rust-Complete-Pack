# Drop trait

**Level:** ⭐ BEGINNER

**Objective:** Implémenter Drop pour nettoyer les ressources

**Problem:**

Implémentez `impl Drop for MyStruct { fn drop(&mut self) { } }`.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Utile pour fermer fichiers, connexions, etc.
- Drop est appelé automatiquement quand une valeur sort de portée.
