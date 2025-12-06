# WASM & Embedded - Rust Partout! 🌐

## Learning Objectives

- Compiler Rust vers WebAssembly
- Utiliser Rust pour embedded
- Voir des exemples COOL

## Core Explanation

### For Absolute Beginners - C'est Comme un Passeport Universel! 🛂

Imaginez que vous avez un **passeport universel** qui vous permet d'aller:
- Dans un **navigateur web** (WASM)
- Sur un **microcontrôleur** (Embedded)
- Partout!

C'est **exactement** comme Rust fonctionne! C'est **super cool**!

## Schéma Visuel - Rust Partout

```
┌─────────────────────────────────────────┐
│  🌐 RUST = PASSEPORT UNIVERSEL 🌐      │
├─────────────────────────────────────────┤
│                                         │
│  Rust Code (Passeport)                  │
│         │                                │
│         ├─> WASM (Navigateur)           │
│         ├─> Embedded (Microcontrôleur)  │
│         └─> Desktop (Ordinateur)         │
│                                         │
│  Partout! ✅                            │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Passeport Universel" - Votre code Rust peut s'exécuter partout: navigateur (WASM), microcontrôleurs (embedded), ordinateurs (desktop), comme un passeport qui ouvre toutes les portes!

## Code Examples

### Example 1: WASM Basique

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
```

## Official Resources

- [wasm-pack](https://rustwasm.github.io/wasm-pack/)

