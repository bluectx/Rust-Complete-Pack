# Supertrait (trait inheritance)

**Level:** ⭐ BEGINNER

**Objective:** Créer un trait qui exige un autre trait

**Problem:**

Écrivez `trait Animal: std::fmt::Display { }`.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Les traits peuvent s'étendre les uns les autres.
- `: OtherTrait` signifie que tous les implémenteurs doivent impl OtherTrait.
