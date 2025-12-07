# Clone d'un Rc (augmente le count)

**Level:** ⭐ BEGINNER

**Objective:** Cloner un Rc pour augmenter le compte des références

**Problem:**

Clonez un Rc plusieurs fois et observez le comportement.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Cloner un Rc est cheap ; c'est just un pointeur.
- `.clone()` sur Rc incrémente le compte, ne copie pas la donnée.
