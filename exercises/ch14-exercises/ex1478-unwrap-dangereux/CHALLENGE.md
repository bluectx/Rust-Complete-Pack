# unwrap() (dangereux!)

**Level:** ⭐ BEGINNER

**Objective:** Utiliser unwrap() pour extraire Ok ou paniquer

**Problem:**

Appelez unwrap() sur un Result Ok. Expliquez ce qui se passe avec Err.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- unwrap() n'est sûr que si vous êtes certain du Ok.
- `.unwrap()` retourne T ou panique.
