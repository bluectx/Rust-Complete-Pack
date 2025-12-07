# Safe abstractions over unsafe

**Level:** ⭐ BEGINNER

**Objective:** Créer une API safe autour d'unsafe code

**Problem:**

Écrivez une struct safe qui contient un unsafe block bien délimité.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Localiser et documenter le danger.
- Le but : hide unsafe behind safe API.
