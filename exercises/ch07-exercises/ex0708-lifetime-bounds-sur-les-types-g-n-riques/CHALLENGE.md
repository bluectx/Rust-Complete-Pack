# Lifetime bounds sur les types génériques

**Level:** ⭐⭐ INTERMEDIATE

**Objective:** Ajouter des bounds de lifetime

**Problem:**

Écrivez `fn f<'a, T: 'a>(x: &'a T)`.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Contrôle fin du temps de vie.
- `T: 'a` = tous les refs dans T doivent vivre au moins 'a.
