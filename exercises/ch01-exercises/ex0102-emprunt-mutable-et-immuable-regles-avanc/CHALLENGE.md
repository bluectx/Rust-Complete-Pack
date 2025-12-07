# Emprunt mutable et immuable (regles avancées)

**Level:** ⭐ BEGINNER

**Objective:** Gérer des cas complexes de borrow checking

**Problem:**

Créez plusieurs &mut et & sur la même variable, respectez l'ordre de portée.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Comprendre le NLL aide à écrire du code plus flexible.
- Le NLL (Non-Lexical Lifetimes) affecte quand une ref se termine.
