# Unsafe blocks

**Level:** ⭐ BEGINNER

**Objective:** Utiliser unsafe avec soin et documentation

**Problem:**

Écrivez un unsafe block avec un commentaire explicant pourquoi c'est sûr.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- unsafe ne devrait être utilisé que avec justification.
- unsafe { } désactive les vérifications du borrow checker.
