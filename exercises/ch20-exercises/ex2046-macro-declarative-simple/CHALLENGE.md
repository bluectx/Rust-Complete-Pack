# Macro declarative simple

**Level:** ⭐⭐ INTERMEDIATE

**Objective:** Créer une macro avec macro_rules!

**Problem:**

Écrivez `macro_rules! my_macro { ($x:expr) => { println!("{}", $x); } }`.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Les macros sont du "copy-paste intelligent".
- macro_rules! prend patterns et les expands.
