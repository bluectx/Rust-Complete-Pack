# Zeroizing data

**Level:** ⭐⭐⭐ ADVANCED

**Objective:** Sécuriser les données sensibles (passwords, clés)

**Problem:**

Utilisez zeroize crate pour effacer la mémoire après utilisation.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Sécurité : ne pas laisser traîner les secrets.
- zeroize_on_drop = efface avant destruction.
