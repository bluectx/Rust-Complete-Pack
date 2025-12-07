# Self lifetime dans les traits

**Level:** ⭐ BEGINNER

**Objective:** Utiliser des lifetimes associés aux traits

**Problem:**

Écrivez un trait avec une méthode retournant une ref avec lifetime lié.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Nécessaire pour les iterators et les builders.
- Certains traits doivent annoter les lifetimes du return.
