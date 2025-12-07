# Struct générique

**Level:** ⭐ BEGINNER

**Objective:** Créer une struct avec paramètres de type

**Problem:**

Créez `struct Boite<T> { contenu: T }` et instanciez-la avec différents types.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- Chaque instanciation de Boite<T> est un type distinct.
- `struct Name<T> { field: T, ... }`
