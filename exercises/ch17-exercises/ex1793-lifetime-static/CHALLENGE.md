# Lifetime "static"

**Level:** ⭐ BEGINNER

**Objective:** Comprendre le lifetime statique

**Problem:**

Créez une variable avec lifetime 'static (comme un literal string).

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Les string literals ont 'static par défaut.
- `'static` signifie que la référence vit pour le reste du programme.
