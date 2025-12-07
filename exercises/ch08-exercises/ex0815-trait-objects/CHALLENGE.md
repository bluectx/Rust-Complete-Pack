# Trait objects

**Level:** ⭐⭐ INTERMEDIATE

**Objective:** Utiliser dyn Trait pour le polymorphisme dynamique

**Problem:**

Créez `let animals: Vec<Box<dyn Animal>> = vec![...];`.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Alternative aux génériques quand les types varient.
- `dyn Trait` dispatche dynamiquement au runtime.
