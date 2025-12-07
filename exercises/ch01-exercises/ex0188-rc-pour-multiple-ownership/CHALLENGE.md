# Rc pour multiple ownership

**Level:** ⭐ BEGINNER

**Objective:** Partager l'ownership avec Rc<T>

**Problem:**

Créez un Rc, clonez-le, et utilisez-le dans deux portées différentes.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Rc permet multiple ownership read-only.
- `Rc<T>` compte les références ; détruit quand count = 0.
