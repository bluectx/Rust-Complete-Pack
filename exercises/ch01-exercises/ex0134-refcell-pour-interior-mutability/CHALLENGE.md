# RefCell pour interior mutability

**Level:** ⭐ BEGINNER

**Objective:** Modifier des données derrière une référence immuable avec RefCell

**Problem:**

Utilisez `RefCell<T>` avec `.borrow_mut()` pour mutabilité intérieure.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Utile quand le borrow checker est trop strict.
- RefCell repousse le borrow checking au runtime.
