# Filtrage avec filter

**Level:** ⭐ BEGINNER

**Objective:** Sélectionner des éléments avec filter()

**Problem:**

Créez un vec, gardez seulement les nombres pairs avec .filter().

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- filter() retourne un itérateur (lazy).
- `.filter(|x| x % 2 == 0)` sélectionne les éléments matching.
