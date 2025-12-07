# Blanket implementations

**Level:** ⭐⭐ INTERMEDIATE

**Objective:** Implémenter un trait pour tous les types satisfaisant une condition

**Problem:**

Écrivez `impl<T: Display> ToString for T { }`.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Couverture complète de l'espace des types.
- Les impls génériques appliquent à beaucoup de types.
