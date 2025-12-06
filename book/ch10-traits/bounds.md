# Trait Bounds - Contraintes Magiques! 🎯

## Learning Objectives

- Utiliser des trait bounds dans les fonctions génériques (c'est puissant!)
- Comprendre la syntaxe T: Trait
- Utiliser plusieurs bounds

## Core Explanation

### For Absolute Beginners - C'est Comme un Filtre! 🔍

Imaginez un **filtre** 🔍:
- **Trait bound** = Un filtre qui dit "seulement les types qui ont ce trait"
- Vous filtrez les types acceptables
- C'est **super pratique** pour la type safety!

C'est **exactement** comme les trait bounds fonctionnent! C'est **super puissant**!

## Schéma Visuel - Trait Bounds

```
┌─────────────────────────────────────────┐
│  🔍 TRAIT BOUNDS = FILTRE 🔍           │
├─────────────────────────────────────────┤
│                                         │
│  fn afficher<T: Display>(item: T)       │
│         │                                │
│         └─> Filtre: seulement Display!  │
│                                         │
│  Types filtrés! ✅                      │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Filtre" - Les trait bounds sont comme un filtre: ils acceptent seulement les types qui ont le trait requis!

## Code Examples

```rust
use std::fmt::Display;

fn afficher<T: Display>(item: T) {
    println!("{}", item);
}

fn afficher_multiple<T: Display + Clone>(item: T) {
    let copie = item.clone();
    println!("Original: {}, Copie: {}", item, copie);
}
```

## Official Resources

- [@official Rust Book - Trait Bounds](https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bounds)

