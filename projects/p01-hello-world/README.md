# Projet 1: Hello World CLI

## Description

Un programme simple qui affiche un message de salutation en lisant un nom depuis les arguments de ligne de commande.

## Objectifs d'Apprentissage

- Utiliser les arguments CLI avec clap
- Gérer les valeurs optionnelles
- Écrire des tests unitaires
- Gérer les erreurs proprement

## Utilisation

```bash
# Avec un nom
cargo run -- Alice

# Sans nom (utilise "Monde" par défaut)
cargo run
```

## Tests

```bash
cargo test
```

## Structure

- `src/main.rs` : Code principal
- `Cargo.toml` : Configuration et dépendances

