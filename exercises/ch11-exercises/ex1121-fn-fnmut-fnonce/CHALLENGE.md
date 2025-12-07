# Fn, FnMut, FnOnce

**Level:** ⭐⭐ INTERMEDIATE

**Objective:** Comprendre les trois traits de closure

**Problem:**

Écrivez des closures qui impl Fn, FnMut, FnOnce différemment.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Choisir le bon trait affecte la capture et la réutilisabilité.
- Fn(&T), FnMut(&mut T), FnOnce(T) = trois niveaux de capture.
