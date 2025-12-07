# should_panic pour tester les paniques

**Level:** ⭐ BEGINNER

**Objective:** Vérifier qu'une fonction panique attendue

**Problem:**

Écrivez `#[test] #[should_panic] fn it_panics() { panic!("oops"); }`

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Utile pour valider les comportements d'erreur.
- `#[should_panic]` teste qu'une fonction panique.
