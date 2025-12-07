# Destructuration dans les patterns

**Level:** ⭐⭐ INTERMEDIATE

**Objective:** Extraire des données imbriquées avec patterns

**Problem:**

Destructurez une struct imbriquée dans un match.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Les patterns peuvent aller en profondeur.
- `match { MyEnum::Variant(MyStruct { field, .. }) => ... }`.
