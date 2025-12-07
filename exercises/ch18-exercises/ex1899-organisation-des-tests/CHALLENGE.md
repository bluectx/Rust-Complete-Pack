# Organisation des tests

**Level:** ⭐ BEGINNER

**Objective:** Organiser les tests dans des modules

**Problem:**

Créez un module `#[cfg(test)] mod tests { }` avec plusieurs tests.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Cela maintient le code de test séparé.
- `#[cfg(test)]` compile le module seulement pour les tests.
