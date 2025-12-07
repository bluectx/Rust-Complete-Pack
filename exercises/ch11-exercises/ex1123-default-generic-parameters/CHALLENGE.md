# Default generic parameters

**Level:** ⭐⭐ INTERMEDIATE

**Objective:** Ajouter des paramètres génériques par défaut

**Problem:**

Écrivez `struct MyStruct<T = i32> { t: T }`.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Rend l'utilisation plus conviviale.
- `<T = Type>` = T par défaut Type si non spécifié.
