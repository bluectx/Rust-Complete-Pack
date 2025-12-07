# Box pour heap allocation

**Level:** ⭐ BEGINNER

**Objective:** Allouer sur le heap avec Box<T>

**Problem:**

Créez `let b = Box::new(5);` et utilisez la déréférence.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Box transfère l'ownership de manière unique.
- `Box<T>` alloue T sur le heap.
