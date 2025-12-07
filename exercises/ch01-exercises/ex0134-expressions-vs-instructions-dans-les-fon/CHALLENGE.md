# Expressions vs instructions dans les fonctions

**Level:** ⭐ BEGINNER

**Objective:** Distinguer expressions (retournent une valeur) et instructions (ne retournent rien)

**Problem:**

Écrivez une fonction avec une expression (sans ;) et une instruction (avec ;).

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Pas de ; à la fin = expression (retourne la valeur) ; ; à la fin = instruction.
- `let x = 5 + 6;` est une instruction. `5 + 6` est une expression.
