# Evasion de lifetime (borrow checker)

**Level:** ⭐ BEGINNER

**Objective:** Créer une situation qui viole les lifetimes

**Problem:**

Essayez de créer une référence dangereuse. Montrez comment Rust la rejette.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Rust valide les lifetimes à la compilation, pas à l'exécution.
- Le borrow checker empêche les dangling references.
