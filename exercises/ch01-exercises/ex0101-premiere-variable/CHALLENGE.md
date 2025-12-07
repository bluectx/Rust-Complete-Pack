# Première variable immuable

**Level:** ⭐ Beginner

**Objective:** Déclarer une variable et comprendre l'immuabilité par défaut en Rust

**Problem:**

Déclarez une variable `x` avec une valeur entière. Essayez de la modifier. Expliquez pourquoi cela échoue.

**Example Runs:**

```bash
$ cargo run
# Le code devrait compiler mais échouer à l'exécution si vous essayez de modifier x
# Ou le code ne devrait pas compiler si vous essayez de modifier une variable immuable
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- L'immuabilité est un trait fondamental de Rust ; il faut explicitement utiliser `mut` pour la modifier.
- En Rust, les variables sont immuables par défaut. C'est un choix délibéré pour la sécurité.

