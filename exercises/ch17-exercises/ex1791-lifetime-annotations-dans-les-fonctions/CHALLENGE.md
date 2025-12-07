# Lifetime annotations dans les fonctions

**Level:** ⭐ BEGINNER

**Objective:** Annotter explicitement les lifetimes

**Problem:**

Écrivez `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str`.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- 'a relie les lifetimes entre les paramètres et le retour.
- `'a` est un paramètre de lifetime.
