# String vs &str

**Level:** ⭐ BEGINNER

**Objective:** Comprendre la différence entre String et &str

**Problem:**

Créez une String et une &str literal. Utilisez-les dans une fonction.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- &str est l'idiom Rust pour les chaînes immuables.
- String = heap, mutable. &str = slice, immuable.
