# Ownership et fonctions

**Level:** ⭐ BEGINNER

**Objective:** Comprendre comment l'ownership change quand on passe des valeurs à des fonctions

**Problem:**

Passez une String à une fonction. Essayez d'utiliser la variable après. Expliquez.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Ownership peut être transféré ; il faut la retourner pour la récupérer.
- Passer une value à une fonction la déplace ; la fonction devient propriétaire.
