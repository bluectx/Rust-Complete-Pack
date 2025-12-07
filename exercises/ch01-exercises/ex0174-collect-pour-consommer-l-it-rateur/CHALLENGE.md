# collect() pour consommer l'itérateur

**Level:** ⭐ BEGINNER

**Objective:** Transformer un itérateur en collection

**Problem:**

Utilisez .map().filter().collect() pour créer un nouveau Vec.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- collect() force l'évaluation ; il faut souvent annoter le type.
- `.collect()` consume l'itérateur et crée une collection.
