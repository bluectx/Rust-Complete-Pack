# Macros Procédurales - Code qui Génère du Code! 🎨

## Learning Objectives

- Comprendre les macros procédurales comme des générateurs de code
- Utiliser derive macros
- Voir des exemples COOL

## Core Explanation

### For Absolute Beginners - C'est Comme un Assistant qui Écrit pour Vous! ✍️

Imaginez que vous voulez créer une struct avec beaucoup de méthodes:
- **Macro procédurale** = Un assistant qui écrit automatiquement toutes les méthodes!
- Vous dites juste `#[derive(Debug, Clone)]` et l'assistant fait le reste!

C'est **exactement** comme les macros procédurales fonctionnent! C'est **super pratique**!

## Schéma Visuel - Macros Procédurales

```
┌─────────────────────────────────────────┐
│  🎨 MACROS PROCÉDURALES = ASSISTANT 🎨 │
├─────────────────────────────────────────┤
│                                         │
│  Vous écrivez:                          │
│  #[derive(Debug)]                       │
│  struct User { ... }                    │
│                                         │
│  Assistant génère automatiquement:      │
│  impl Debug for User { ... }            │
│                                         │
│  C'est magique! ✨                      │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Assistant Automatique" - Un assistant qui écrit le code pour vous automatiquement, vous n'avez qu'à demander avec `#[derive(...)]`!

## Code Examples

### Example 1: Derive Macro (Super Facile!)

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
}

fn main() {
    let user = User {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    
    // Debug généré automatiquement!
    println!("{:?}", user);
}
```

## Official Resources

- [@official Rust Book - Procedural Macros](https://doc.rust-lang.org/book/ch19-06-macros.html#procedural-macros-for-generating-code-from-attributes)

