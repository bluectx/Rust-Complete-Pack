# Pin et self-referential structs

**Level:** ⭐ BEGINNER

**Objective:** Comprendre Pin pour les futures auto-référentes

**Problem:**

Utilisez `Box::pin()` pour créer une Future pinned.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Essentiel pour les futures auto-référentes.
- Pin garantit qu'une Future ne bouge pas en mémoire.
