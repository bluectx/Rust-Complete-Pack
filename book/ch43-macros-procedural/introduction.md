# Macros Procédurales - Introduction

## Learning Objectives

- Comprendre les macros procédurales (c'est avancé!)
- Créer des derive macros
- Utiliser les attribute macros
- Voir les exemples

## Core Explanation

### For Absolute Beginners - C'est Comme un Assistant Programmé! 🤖

Imaginez un **assistant programmé** 🤖:
- **Macros procédurales** = Un assistant qui écrit du code pour vous
- Vous dites ce que vous voulez → L'assistant génère le code!
- C'est **super puissant** mais **super avancé**!

C'est **exactement** comme les macros procédurales fonctionnent! C'est **super magique**!

## Schéma Visuel - Macros Procédurales

```
┌─────────────────────────────────────────┐
│  🤖 MACROS PROCÉDURALES = ASSISTANT 🤖 │
├─────────────────────────────────────────┤
│                                         │
│  #[derive(Debug, Clone)]                │
│  struct User { ... }                    │
│         │                                │
│         ▼ Assistant génère               │
│  impl Debug for User { ... }            │
│  impl Clone for User { ... }           │
│                                         │
│  Code automatique avancé! ✨            │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Assistant Programmé" - Les macros procédurales sont comme un assistant programmé: vous demandez avec #[derive(...)], et il génère le code automatiquement!

## Code Examples

### Example 1: Utiliser des Derive Macros

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
}
```

## Official Resources

- [@official Rust Book - Procedural Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)

