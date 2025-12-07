# Custom error types

**Level:** ⭐ BEGINNER

**Objective:** Créer un type d'erreur custom avec impl Display et Error

**Problem:**

Implémentez `impl Display for MyError { }` et `impl Error for MyError { }`.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Les erreurs custom offrent plus d'info que les Strings.
- Custom errors doivent impl Display et Error traits.
