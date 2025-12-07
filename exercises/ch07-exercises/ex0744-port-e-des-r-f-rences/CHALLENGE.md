# Portée des références

**Level:** ⭐ BEGINNER

**Objective:** Comprendre où une référence est valide

**Problem:**

Créez une &mut, utilisez-la, puis créez une autre &mut après. Devrait compiler.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Rust utilise le NLL (Non-Lexical Lifetimes).
- Une référence est valide jusqu'à sa dernière utilisation, pas jusqu'à la fin du bloc.
