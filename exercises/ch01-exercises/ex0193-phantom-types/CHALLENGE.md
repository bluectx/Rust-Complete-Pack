# Phantom types

**Level:** ⭐ BEGINNER

**Objective:** Utiliser PhantomData pour les types sans données

**Problem:**

Utilisez `struct Wrapper<T>(PhantomData<T>)` pour le typage.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Marqueurs de type.
- PhantomData = type sans overhead mémoire.
