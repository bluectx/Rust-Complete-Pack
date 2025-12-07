# Copie vs déplacement

**Level:** ⭐ BEGINNER

**Objective:** Comprendre que les types Copy (i32, bool, etc.) se copient au lieu de se déplacer

**Problem:**

Assignez un i32 à x, puis à y. Utilisez x après. Comparez avec une String.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Copy trait rend les types copiables ; ownership rule #2.
- Les types Copy sont petits et se copient implicitement. String ne l'est pas.
