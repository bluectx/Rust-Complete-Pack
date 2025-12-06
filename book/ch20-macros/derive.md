# Derive Macros - Code Automatique! 🎯

## Learning Objectives

- Utiliser derive pour générer du code (c'est magique!)
- Comprendre les derives courants
- Créer des derives personnalisés
- Voir les exemples

## Core Explanation

### For Absolute Beginners - C'est Comme un Assistant! 🤖

Imaginez un **assistant** 🤖 qui écrit du code pour vous:
- **derive** = L'assistant qui génère automatiquement
- Vous dites juste `#[derive(Debug)]` → L'assistant fait le reste!
- C'est **super pratique** et **super rapide**!

C'est **exactement** comme derive fonctionne! C'est **super magique**!

## Schéma Visuel - Derive

```
┌─────────────────────────────────────────┐
│  🤖 DERIVE = ASSISTANT 🤖              │
├─────────────────────────────────────────┤
│                                         │
│  #[derive(Debug)]                       │
│  struct User { ... }                    │
│         │                                │
│         ▼ Assistant génère              │
│  impl Debug for User { ... }            │
│                                         │
│  Code automatique! ✨                   │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Assistant" - derive est comme un assistant: vous demandez avec `#[derive(...)]`, il génère le code automatiquement!

## Code Examples

### Example 1: Derives Standards

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 0, y: 0 };
    let p2 = p1.clone();  // Clone généré automatiquement
    println!("{:?}", p1);  // Debug généré automatiquement
    assert_eq!(p1, p2);  // PartialEq généré automatiquement
}
```

### Example 2: Derive avec Serde

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    host: String,
    port: u16,
}

fn main() {
    let config = Config {
        host: "localhost".to_string(),
        port: 8080,
    };
    
    let json = serde_json::to_string(&config).unwrap();
    println!("{}", json);
}
```

### Example 3: Derives Disponibles

```rust
// Standards
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]

// Externes
#[derive(Serialize, Deserialize)]  // serde
#[derive(Error)]  // thiserror
```

## Trait Derivable

Un trait est "derivable" si on peut utiliser `#[derive(Trait)]` pour générer automatiquement l'implémentation.

## Official Resources

- [@official Rust Book - Derive](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)

## Security Notes

Les derives génèrent du code à la compilation :
- Vérifier le code généré
- Tester les cas limites
- Documenter le comportement
