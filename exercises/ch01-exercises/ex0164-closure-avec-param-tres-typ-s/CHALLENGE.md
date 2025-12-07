# Closure avec paramètres typés

**Level:** ⭐ BEGINNER

**Objective:** Annoter les types dans une closure

**Problem:**

Créez `let add = |x: i32, y: i32| -> i32 { x + y };`

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Les annotations rendent l'intention explicite.
- `|params: Type| -> ReturnType { body }`
