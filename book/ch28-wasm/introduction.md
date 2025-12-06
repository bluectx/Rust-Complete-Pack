# WebAssembly avec Rust

## Learning Objectives

- Compiler Rust vers WebAssembly
- Utiliser wasm-pack
- Intégrer WASM avec JavaScript
- Optimiser les performances WASM

## Key Vocabulary

| Term | Definition |
|------|-----------|
| WASM | WebAssembly, format binaire pour le web |
| wasm-pack | Outil pour packager Rust en WASM |
| wasm-bindgen | Bindings entre Rust et JavaScript |
| Target | Architecture de compilation |

## Core Explanation

### For Absolute Beginners - C'est Comme Concept! 📚

Ce chapitre vous enseignera les concepts fondamentaux de manière simple et progressive.

C'est **exactement** comme ça fonctionne! C'est **super pratique**!

## Schéma Visuel - Concept

```
┌─────────────────────────────────────────┐
│  📚 CONCEPT = Concept 📚 │
├─────────────────────────────────────────┤
│                                         │
│  Concept principal                      │
│         │                                │
│         ▼ Explication                    │
│  ┌─────────────┐                        │
│  │ Concept │ → Fonctionne! ✅│
│  └─────────────┘                        │
│                                         │
│  Simple et puissant! ✅                 │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Concept" - Ce chapitre vous enseignera les concepts fondamentaux de manière simple et progressive.

## For Absolute Beginners

WebAssembly permet d'exécuter du code Rust dans le navigateur web, avec des performances proches du natif. C'est comme compiler votre code Rust pour qu'il fonctionne partout où JavaScript fonctionne.

**Avantages :**
- Performance native dans le navigateur
- Réutiliser le code Rust
- Sécurité (sandbox)
- Support multi-plateforme

## Installation

```bash
# Installer wasm-pack
cargo install wasm-pack

# Ajouter la target WASM
rustup target add wasm32-unknown-unknown
```

## Example 1: Premier Projet WASM

**Cargo.toml :**

```toml
[package]
name = "wasm-example"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

**src/lib.rs :**

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

**Compiler :**

```bash
wasm-pack build --target web
```

## Example 2: Utilisation dans JavaScript

```javascript
import init, { add, greet } from './pkg/wasm_example.js';

async function run() {
    await init();
    console.log(add(5, 3));  // 8
    console.log(greet("World"));  // "Hello, World!"
}
```

## Optimisations

```bash
# Build optimisé
wasm-pack build --target web --release

# Minifier
wasm-opt -Os pkg/wasm_example_bg.wasm -o pkg/wasm_example_bg.wasm
```

## Official Resources

- [wasm-pack](https://rustwasm.github.io/wasm-pack/)
- [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)
- [The wasm-pack Book](https://rustwasm.github.io/docs/wasm-pack/)

## Security Notes

WASM est exécuté dans un sandbox :
- Pas d'accès direct au système de fichiers
- Pas d'accès réseau direct
- Isolation du reste du système
- Permissions contrôlées par le navigateur
