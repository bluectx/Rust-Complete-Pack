# Visibility avec pub in

**Level:** ⭐ BEGINNER

**Objective:** Restreindre la visibilité à des modules spécifiques

**Problem:**

Utilisez `pub(in path) fn ...` pour une visibilité limitée.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Contrôle fin de l'API.
- `pub(crate)` = visible dans la crate ; `pub(super)` = parent.
