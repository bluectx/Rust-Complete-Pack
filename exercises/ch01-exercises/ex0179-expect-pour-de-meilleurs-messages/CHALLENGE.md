# expect() pour de meilleurs messages

**Level:** ⭐ BEGINNER

**Objective:** Utiliser expect() pour des messages de panique personnalisés

**Problem:**

Utilisez `.expect("message custom")` au lieu de unwrap().

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- expect() est préféré à unwrap() pour le debugging.
- `.expect(msg)` panique avec msg au lieu d'un générique.
