# Qualified syntax

**Level:** ⭐ BEGINNER

**Objective:** Désambiguïser les appels de méthode

**Problem:**

Utilisez `<Type as Trait>::method(self)` pour clarifier.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Syntaxe explicite quand l'inférence est ambiguë.
- Utile quand plusieurs traits définissent le même nom de méthode.
