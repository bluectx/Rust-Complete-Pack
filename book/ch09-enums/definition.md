# Définition d'Enums - C'est Super Facile! 🎯

## Learning Objectives

- Comprendre ce qu'est un enum (c'est simple!)
- Définir des enums simples
- Utiliser des enums avec match
- Comprendre les avantages des enums

## Core Explanation

### For Absolute Beginners - C'est Comme un Menu! 🍔

Imaginez un **menu McDo** 🍔:
- **Enum** = La liste des options (Burger, Frites, Boisson)
- Chaque option = une variante de l'enum
- Vous choisissez UNE option à la fois!

C'est **exactement** comme les enums fonctionnent! C'est **super pratique**!

## Schéma Visuel - Enum

```
┌─────────────────────────────────────────┐
│  🍔 ENUM = MENU MCDO 🍔                │
├─────────────────────────────────────────┤
│                                         │
│  enum Repas {                           │
│    ├─> Burger 🍔                        │
│    ├─> Frites 🍟                        │
│    ├─> Boisson 🥤                       │
│    └─> Dessert 🍰                       │
│  }                                       │
│                                         │
│  Vous choisissez UNE option! ✅        │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "McDo" - Un enum est comme un menu McDo: plusieurs options, vous en choisissez une!

## Code Examples

### Example 1: Enum Basique

```rust
enum Direction {
    Nord,
    Sud,
    Est,
    Ouest,
}

fn main() {
    let direction = Direction::Nord;
    
    match direction {
        Direction::Nord => println!("Aller vers le nord"),
        Direction::Sud => println!("Aller vers le sud"),
        Direction::Est => println!("Aller vers l'est"),
        Direction::Ouest => println!("Aller vers l'ouest"),
    }
}
```

### Example 2: Enum avec Données

```rust
enum Message {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
    ChangerCouleur(i32, i32, i32),
}

fn main() {
    let msg = Message::Deplacer { x: 10, y: 20 };
    
    match msg {
        Message::Quitter => println!("Quitter"),
        Message::Deplacer { x, y } => println!("Déplacer à ({}, {})", x, y),
        Message::Ecrire(text) => println!("Écrire: {}", text),
        Message::ChangerCouleur(r, g, b) => println!("Couleur: RGB({}, {}, {})", r, g, b),
    }
}
```

## Avantages des Enums

- **Type safety** : Impossible d'avoir une valeur invalide
- **Exhaustivité** : match doit couvrir tous les cas
- **Flexibilité** : Peut contenir des données
- **Clarté** : Code plus expressif

## Pièges Courants

1. **Oublier un cas dans match** : Le compilateur vous avertit!
2. **Confondre avec structs** : Enums = choix, Structs = groupement
3. **Pattern matching** : Toujours utiliser match pour les enums

## Official Resources

- [@official Rust Book - Enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)

