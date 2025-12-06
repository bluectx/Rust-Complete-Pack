# Pattern Matching avec Enums - C'est Super Puissant! 🎯

## Learning Objectives

- Utiliser match avec des enums (c'est exhaustif!)
- Déstructurer les enums
- Gérer tous les cas possibles
- Utiliser if let et while let

## Core Explanation

### For Absolute Beginners - C'est Comme un Menu avec Choix! 🍔

Imaginez un **menu McDo** 🍔:
- **match** = Vous devez choisir parmi TOUTES les options
- Le compilateur vérifie que vous n'oubliez rien!
- C'est **super sûr** et **super pratique**!

## Schéma Visuel - Pattern Matching

```
┌─────────────────────────────────────────┐
│  🍔 MATCH = MENU EXHAUSTIF 🍔          │
├─────────────────────────────────────────┤
│                                         │
│  enum Repas {                           │
│    Burger, Frites, Boisson              │
│  }                                       │
│                                         │
│  match repas {                          │
│    Burger  → "Burger!"                  │
│    Frites  → "Frites!"                  │
│    Boisson → "Boisson!"                │
│  }                                       │
│                                         │
│  Tous les cas couverts! ✅              │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "McDo" - match est comme un menu McDo: vous devez traiter TOUTES les options, le compilateur vérifie que rien n'est oublié!

## Code Examples

### Example 1: Match Basique

```rust
enum Direction {
    Nord,
    Sud,
    Est,
    Ouest,
}

fn direction_en_mots(dir: Direction) -> &'static str {
    match dir {
        Direction::Nord => "Nord",
        Direction::Sud => "Sud",
        Direction::Est => "Est",
        Direction::Ouest => "Ouest",
    }
}

fn main() {
    let dir = Direction::Nord;
    println!("Direction: {}", direction_en_mots(dir));
}
```

### Example 2: Match avec Données

```rust
enum Message {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
}

fn traiter_message(msg: Message) {
    match msg {
        Message::Quitter => println!("Au revoir!"),
        Message::Deplacer { x, y } => {
            println!("Déplacer à ({}, {})", x, y);
        }
        Message::Ecrire(text) => {
            println!("Écrire: {}", text);
        }
    }
}
```

### Example 3: if let

```rust
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let option = Some(5);
    
    // Au lieu de match complet
    if let Some(value) = option {
        println!("Valeur: {}", value);
    }
    
    // Équivalent à:
    match option {
        Some(value) => println!("Valeur: {}", value),
        None => {},
    }
}
```

### Example 4: while let

```rust
fn main() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```

## Exhaustivité

Le compilateur Rust vérifie que TOUS les cas sont couverts :

```rust
enum Color {
    Red,
    Green,
    Blue,
}

fn process_color(color: Color) {
    match color {
        Color::Red => {},
        Color::Green => {},
        // ERREUR: Blue manquant!
    }
}
```

## Official Resources

- [@official Rust Book - Pattern Matching](https://doc.rust-lang.org/book/ch06-02-match.html)
