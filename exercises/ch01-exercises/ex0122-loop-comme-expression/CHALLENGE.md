# loop comme expression

**Level:** ⭐ BEGINNER

**Objective:** Retourner une valeur d'une boucle loop

**Problem:**

Utilisez loop pour assigner une valeur à une variable via break valeur;

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- `let x = loop { if condition { break x; } };`
- `break valeur;` interrompt la boucle et retourne valeur.
