# strong_count()

**Level:** ⭐ BEGINNER

**Objective:** Vérifier le nombre de références fortes

**Problem:**

Utilisez `Rc::strong_count()` pour afficher le nombre de refs.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Utile pour le débogage et la compréhension du lifecycle.
- `Rc::strong_count(&rc)` retourne le nombre de clones.
