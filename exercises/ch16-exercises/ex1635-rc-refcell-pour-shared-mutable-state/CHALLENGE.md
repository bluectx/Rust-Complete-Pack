# Rc + RefCell pour shared mutable state

**Level:** ⭐⭐ INTERMEDIATE

**Objective:** Combiner ownership partagé et mutabilité intérieure

**Problem:**

Créez `Rc<RefCell<T>>` et modifiez via plusieurs références.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Idiom important pour les structures de données complexes.
- Rc + RefCell = partage avec mutations.
