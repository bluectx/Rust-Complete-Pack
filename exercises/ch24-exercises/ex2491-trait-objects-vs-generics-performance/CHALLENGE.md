# Trait objects vs generics (performance)

**Level:** ⭐⭐⭐ ADVANCED

**Objective:** Comprendre les compromis polymorphisme

**Problem:**

Comparez dyn Trait (monomorphization) vs <T: Trait> (vtables).

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Choisir en fonction des besoins.
- Trait objects = vtables ; Génériques = code duplication.
