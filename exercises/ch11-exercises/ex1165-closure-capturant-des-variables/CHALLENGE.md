# Closure capturant des variables

**Level:** ⭐ BEGINNER

**Objective:** Utiliser des variables du contexte dans une closure

**Problem:**

Créez une variable x et une closure qui la capture et l'utilise.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- La capture peut être immutable (&), mutable (&mut), ou mover (move).
- Les closures ont accès aux variables du scope environnant.
