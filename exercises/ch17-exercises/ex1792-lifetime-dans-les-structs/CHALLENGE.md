# Lifetime dans les structs

**Level:** ⭐ BEGINNER

**Objective:** Ajouter des references à une struct avec lifetimes

**Problem:**

Créez `struct Borrows<'a> { s: &'a str }`.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Cela assure que la référence vit assez longtemps.
- Les structs avec références doivent spécifier les lifetimes.
