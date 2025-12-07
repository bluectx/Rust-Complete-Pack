# Propager les erreurs avec ?

**Level:** ⭐ BEGINNER

**Objective:** Utiliser l'opérateur ? pour propager les erreurs

**Problem:**

Écrivez une fonction qui utilise ? pour retourner une erreur à l'appelant.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- ? est le sucre syntaxique pour Err(e) => return Err(e).
- `let result = risky_op()?;` retourne immédiatement l'erreur si Err.
