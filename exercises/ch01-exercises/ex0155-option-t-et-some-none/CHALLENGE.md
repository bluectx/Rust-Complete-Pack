# Option<T> et Some/None

**Level:** ⭐ BEGINNER

**Objective:** Comprendre l'enum Option pour représenter une valeur optionnelle

**Problem:**

Créez une fonction qui retourne Option<i32>. Utilisez match pour gérer Some et None.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Option prévient les erreurs null ; c'est idiomatique Rust.
- `Option<T>` = Some(T) ou None. Alternative à null.
