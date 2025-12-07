# Timing-safe comparisons

**Level:** ⭐⭐⭐ ADVANCED

**Objective:** Comparer des données sensibles sans timing attacks

**Problem:**

Utilisez `subtle::ConstantTimeComparison` pour comparer les mots de passe.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Cryptographe doit être constant-time.
- Timing attack = deviner basé sur la durée.
