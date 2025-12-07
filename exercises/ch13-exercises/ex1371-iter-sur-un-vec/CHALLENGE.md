# iter() sur un Vec

**Level:** ⭐ BEGINNER

**Objective:** Utiliser la méthode iter() pour créer un itérateur

**Problem:**

Créez un vec et utilisez .iter() pour boucler. Comparez avec for.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Les itérateurs sont lazy ; rien n'est exécuté jusqu'au consommateur.
- `.iter()` crée un itérateur immuable.
