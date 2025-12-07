# Move vs Copy dans les génériques

**Level:** ⭐⭐ INTERMEDIATE

**Objective:** Comprendre comment les génériques gèrent le transfert

**Problem:**

Créez une fonction générique qui prend ownership vs une qui prend &T.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Les génériques doivent être intentionnels sur ownership.
- `fn f<T>(x: T)` move ; `fn f<T>(x: &T)` emprunt.
