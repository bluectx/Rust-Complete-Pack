# Cargo Avancé

## Learning Objectives

- Maîtriser les commandes cargo
- Gérer les dépendances
- Créer des workspaces
- Publier des crates
- Utiliser les features

## Key Vocabulary

| Term | Definition |
|------|-----------|
| Cargo | Gestionnaire de paquets et build tool Rust |
| Crate | Unité de compilation Rust |
| Dependency | Bibliothèque externe utilisée |
| Workspace | Collection de crates liés |
| Feature | Fonctionnalité optionnelle d'une crate |

## Core Explanation

### For Absolute Beginners - C'est Comme une Boîte à Outils! 🧰

Cargo est comme une **boîte à outils** 🧰 complète:
- **Gestionnaire de paquets** : Télécharge et gère les bibliothèques
- **Build tool** : Compile votre code
- **Test runner** : Lance les tests
- **Documentation** : Génère la documentation

C'est **exactement** comme cargo fonctionne! C'est **super pratique**!

## Schéma Visuel - Cargo

```
┌─────────────────────────────────────────┐
│  🧰 CARGO = BOÎTE À OUTILS 🧰          │
├─────────────────────────────────────────┤
│                                         │
│  cargo build  → Compiler                │
│  cargo test   → Tester                  │
│  cargo doc    → Documenter              │
│  cargo run    → Exécuter                │
│                                         │
│  Tous les outils en un! ✅             │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Boîte à Outils" - Cargo contient tous les outils nécessaires pour développer en Rust: compiler, tester, documenter, tout en un seul endroit!

## Commandes Cargo Essentielles

### Example 1: Commandes de Base

```bash
# Créer un nouveau projet
cargo new mon_projet

# Compiler
cargo build

# Compiler et exécuter
cargo run

# Lancer les tests
cargo test

# Vérifier sans compiler
cargo check

# Formater le code
cargo fmt

# Linter
cargo clippy

# Générer la documentation
cargo doc --open
```

### Example 2: Gestion des Dépendances

**Cargo.toml :**

```toml
[package]
name = "mon_projet"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
tokio = { version = "1.35", features = ["full"] }

[dev-dependencies]
criterion = "1.0"
```

**Mettre à jour :**

```bash
cargo update
```

### Example 3: Workspaces

**Cargo.toml (racine) :**

```toml
[workspace]
members = [
    "crate1",
    "crate2",
]

[workspace.dependencies]
tokio = "1.35"
```

### Example 4: Features

**Cargo.toml :**

```toml
[features]
default = ["std"]
std = []
no-std = []

[dependencies]
# Dépendances conditionnelles
```

**Utiliser :**

```bash
cargo build --features "feature1,feature2"
cargo build --no-default-features
```

## Profils de Build

```toml
[profile.dev]
opt-level = 0  # Pas d'optimisation

[profile.release]
opt-level = 3  # Optimisation maximale
lto = true     # Link-time optimization
```

## Official Resources

- [@official Rust Book - Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

## Security Notes

- Toujours vérifier les dépendances avec `cargo audit`
- Mettre à jour régulièrement
- Utiliser des versions spécifiques en production
