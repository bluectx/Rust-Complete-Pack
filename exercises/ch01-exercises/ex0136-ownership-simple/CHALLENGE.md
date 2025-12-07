# Ownership simple

**Level:** ⭐ BEGINNER

**Objective:** Comprendre que chaque valeur a un propriétaire unique

**Problem:**

Assignez une String à x, puis à y. Essayez d'utiliser x après. Expliquez l'erreur.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Ownership rule #1 : chaque valeur a un propriétaire.
- Quand on assigne une value complexe, on la déplace. L'ancienne variable n'est plus valide.
