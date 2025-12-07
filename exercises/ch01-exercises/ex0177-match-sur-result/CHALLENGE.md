# match sur Result

**Level:** ⭐ BEGINNER

**Objective:** Gérer Ok et Err avec match

**Problem:**

Appelez une fonction qui retourne Result et patternez Ok et Err.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- match force à gérer les deux cas.
- `match result { Ok(val) => {...}, Err(err) => {...} }`
