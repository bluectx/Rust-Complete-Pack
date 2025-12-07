# Référence immuable

**Level:** ⭐ BEGINNER

**Objective:** Emprunter une valeur sans en prendre ownership

**Problem:**

Créez une référence avec & et passez-la à une fonction. Utilisez la variable originale après.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Les références laissent la propriété à la variable originale.
- & crée une référence immuable ; cela n'emprunte que temporairement.
