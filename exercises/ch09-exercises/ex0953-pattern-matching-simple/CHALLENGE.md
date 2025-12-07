# Pattern matching simple

**Level:** ⭐ BEGINNER

**Objective:** Utiliser match pour différencier les variantes

**Problem:**

Créez une Couleur et utilisez match pour afficher le texte correspondant.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- match est exhaustif ; il faut couvrir toutes les variantes.
- `match value { Pattern1 => {...}, Pattern2 => {...}, ... }`
