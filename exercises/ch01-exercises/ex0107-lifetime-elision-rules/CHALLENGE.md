# Lifetime elision rules

**Level:** ⭐ BEGINNER

**Objective:** Comprendre quand Rust omet les lifetimes

**Problem:**

Écrivez des fonctions sans lifetimes explicites ; Rust les inférera.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Moins d'annotations = code plus lisible quand on peut les omettre.
- Rust a 3 règles d'élision ; la plupart des cas simples l'omettent.
