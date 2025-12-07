# if imbriquée

**Level:** ⭐ BEGINNER

**Objective:** Imbriquer des conditions if

**Problem:**

Vérifiez si un nombre est entre 1 et 100 avec deux conditions if imbriquées.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Trop de nesting rend le code difficile à lire ; préférez && pour les conditions multiples.
- Les blocs if peuvent contenir d'autres if.
