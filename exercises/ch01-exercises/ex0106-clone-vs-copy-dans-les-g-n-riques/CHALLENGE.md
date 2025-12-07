# Clone vs Copy dans les génériques

**Level:** ⭐ BEGINNER

**Objective:** Savoir quand cloner dans les génériques

**Problem:**

Écrivez `fn f<T: Clone>(x: T) -> T { x.clone() }`.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Les bounds contraignent les types génériques.
- Nécessite la bound Clone si vous voulez cloner.
