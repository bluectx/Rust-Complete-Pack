# Étiquettes de boucle

**Level:** ⭐ BEGINNER

**Objective:** Contrôler les boucles extérieures avec des étiquettes

**Problem:**

Utilisez 'outer: pour sortir d'une boucle extérieure depuis une boucle interne.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- `'label: loop { loop { break 'label; } }`
- Préfixez une boucle avec 'label: et utilisez break 'label;
