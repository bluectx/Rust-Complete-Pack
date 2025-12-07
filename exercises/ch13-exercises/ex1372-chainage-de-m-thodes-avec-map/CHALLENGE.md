# Chainage de méthodes avec map

**Level:** ⭐ BEGINNER

**Objective:** Transformer les éléments avec map()

**Problem:**

Créez un vec, doublez chaque nombre avec .map(), affichez.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- map() retourne un nouvel itérateur (lazy).
- `.map(|x| x * 2)` transforme chaque élément.
