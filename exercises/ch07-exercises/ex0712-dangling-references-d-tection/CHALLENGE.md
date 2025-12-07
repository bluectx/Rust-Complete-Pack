# Dangling references (détection)

**Level:** ⭐⭐ INTERMEDIATE

**Objective:** Écrire du code qui violerait les lifetimes et voir Rust le rejeter

**Problem:**

Tentez de retourner une référence à une variable locale.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Comprendre pourquoi est clé à maîtriser les lifetimes.
- Le borrow checker empêche les dangling refs.
