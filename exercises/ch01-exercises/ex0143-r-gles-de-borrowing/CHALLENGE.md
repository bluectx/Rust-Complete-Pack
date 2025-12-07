# Règles de borrowing

**Level:** ⭐ BEGINNER

**Objective:** Comprendre les règles du borrow checker

**Problem:**

Essayez de créer deux &mut simultanément sur la même variable. Expliquez l'erreur.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Ces règles préviennent les data races à la compilation.
- Rule : OU plusieurs & immuables, OU une &mut, pas les deux.
