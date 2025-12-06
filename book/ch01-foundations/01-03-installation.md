# 01-03: Installation de Rust

## Learning Objectives

- Installer rustup (gestionnaire de versions Rust)
- Comprendre les éditions Rust (2021, 2024)
- Vérifier l'installation
- Configurer l'environnement de développement
- Comprendre les outils Rust (rustc, cargo, rustup)

## Key Vocabulary

| Term | Definition |
|------|-----------|
| rustup | Gestionnaire de versions et d'outils Rust officiel |
| rustc | Compilateur Rust |
| cargo | Gestionnaire de paquets et outil de build |
| Edition | Version majeure du langage (2021, 2024) |
| Toolchain | Ensemble d'outils Rust (compilateur, stdlib, etc.) |
| rustfmt | Formateur de code Rust |
| clippy | Linter Rust pour détecter les erreurs et améliorer le code |

## Core Explanation

### For Absolute Beginners

Installer Rust, c'est comme installer un atelier complet pour construire des meubles. Vous obtenez :
- **rustc** : Le compilateur (la scie)
- **cargo** : L'outil de gestion (l'établi)
- **rustup** : Le gestionnaire (l'organisateur d'outils)

**rustup** est l'outil officiel qui installe et gère toutes les versions de Rust. C'est comme un gestionnaire de versions qui vous permet de :
- Installer Rust facilement
- Changer de version si nécessaire
- Mettre à jour Rust
- Installer des composants additionnels

## Installation Step-by-Step

### Linux et macOS

```bash
# Télécharger et installer rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Suivre les instructions à l'écran
# Par défaut, rustup installe la version stable
```

**Après l'installation :**

```bash
# Recharger le shell ou exécuter :
source $HOME/.cargo/env

# Vérifier l'installation
rustc --version
cargo --version
```

### Windows

1. Télécharger rustup-init.exe depuis https://rustup.rs/
2. Exécuter l'installateur
3. Suivre les instructions (choisir "default installation")
4. Redémarrer le terminal

**Vérification :**

```powershell
rustc --version
cargo --version
```

## Code Examples

### Example 1: Vérifier l'Installation

```rust
// Créer un fichier test.rs
fn main() {
    println!("Rust est installé correctement!");
    println!("Version du compilateur: {}", env!("RUSTC_VERSION"));
}
```

**Compilation manuelle :**

```bash
rustc test.rs
./test  # Linux/macOS
test.exe  # Windows
```

**Explanation:**

- `rustc` : Compilateur Rust direct (sans cargo)
- Pour des projets réels, on utilise `cargo` (plus pratique)

### Example 2: Créer un Projet avec Cargo

```bash
# Créer un nouveau projet
cargo new mon_premier_projet

# Structure créée :
# mon_premier_projet/
#   ├── Cargo.toml
#   └── src/
#       └── main.rs
```

**Cargo.toml (créé automatiquement) :**

```toml
[package]
name = "mon_premier_projet"
version = "0.1.0"
edition = "2021"

[dependencies]
```

**src/main.rs (créé automatiquement) :**

```rust
fn main() {
    println!("Hello, world!");
}
```

### Example 3: Éditions Rust

```rust
// Rust 2021 Edition (par défaut actuellement)
// Améliorations: nouvelles préfixes de prélude, meilleure gestion des panics

fn main() {
    // En 2021, certaines fonctions sont automatiquement importées
    let vec = Vec::new(); // Pas besoin de "use std::vec::Vec;"
    println!("{:?}", vec);
}
```

**Changer d'édition dans Cargo.toml :**

```toml
[package]
edition = "2024"  # Quand disponible
```

## Gestion des Versions avec rustup

### Voir les Versions Installées

```bash
rustup show
```

### Installer une Version Spécifique

```bash
# Installer la version stable
rustup install stable

# Installer la version beta
rustup install beta

# Installer nightly (pour les fonctionnalités expérimentales)
rustup install nightly
```

### Changer de Version par Défaut

```bash
rustup default stable
rustup default beta
rustup default nightly
```

### Mettre à Jour Rust

```bash
rustup update
```

## Outils Additionnels

### rustfmt (Formateur de Code)

```bash
# Installer
rustup component add rustfmt

# Formater un projet
cargo fmt
```

### clippy (Linter)

```bash
# Installer
rustup component add clippy

# Vérifier un projet
cargo clippy
```

### rust-analyzer (IDE Support)

```bash
# Pour VS Code, installer l'extension "rust-analyzer"
# Pour d'autres éditeurs, voir: https://rust-analyzer.github.io/
```

## Mini-exercices Rustlings

### Exercice 1: Vérifier l'Installation

```bash
# TODO: Exécuter ces commandes et noter les versions
# rustc --version
# cargo --version
# rustup --version
```

### Exercice 2: Créer un Projet

```bash
# TODO: Créer un projet nommé "test-installation"
# cargo new test-installation
# cd test-installation
# cargo run
```

## Exercises

### Exercise 1: Configuration Initiale

**Level:** ⭐ Beginner

**Challenge:** 
1. Installer Rust avec rustup
2. Vérifier que rustc, cargo et rustup fonctionnent
3. Créer un projet "hello_rust" et l'exécuter

### Exercise 2: Explorer les Outils

**Level:** ⭐⭐ Intermediate

**Challenge:** 
1. Installer rustfmt et clippy
2. Créer un projet avec du code mal formaté
3. Utiliser `cargo fmt` pour le formater
4. Utiliser `cargo clippy` pour trouver des améliorations

## Cheatsheet

```
INSTALLATION RUST
├── rustup → Gestionnaire de versions
├── rustc → Compilateur
├── cargo → Build tool + package manager
└── Éditions: 2015, 2018, 2021, 2024

COMMANDES ESSENTIELLES
├── rustup update        → Mettre à jour
├── cargo new <nom>      → Nouveau projet
├── cargo build          → Compiler
├── cargo run            → Compiler + exécuter
├── cargo test           → Tests
├── cargo fmt            → Formater
└── cargo clippy         → Linter
```

## Common Pitfalls

- ❌ **Mistake:** Oublier de recharger le shell après installation
  ```bash
  # Erreur: command not found: rustc
  ```
  ✅ **Fix:** Exécuter `source $HOME/.cargo/env` ou redémarrer le terminal

- ❌ **Mistake:** Utiliser rustc directement au lieu de cargo
  ```bash
  rustc src/main.rs  # Fonctionne mais pas optimal
  ```
  ✅ **Fix:** Utiliser cargo pour les projets
  ```bash
  cargo build  # Meilleure pratique
  ```

- ❌ **Mistake:** Ne pas mettre à jour rustup régulièrement
  ✅ **Fix:** Exécuter `rustup update` régulièrement pour les dernières versions

## Official Resources

- [@official Rust Installation Guide](https://www.rust-lang.org/tools/install)
- [@official Rust Book - Installation](https://doc.rust-lang.org/book/ch01-01-installation.html)
- [@official rustup Documentation](https://rust-lang.github.io/rustup/)

## Security Notes

- Toujours télécharger rustup depuis https://rustup.rs/ (site officiel)
- Vérifier les checksums si nécessaire
- Ne pas utiliser de versions non officielles
- Mettre à jour régulièrement pour les correctifs de sécurité

