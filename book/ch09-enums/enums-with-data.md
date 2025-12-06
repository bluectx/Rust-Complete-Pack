# Enums avec Données - C'est Super Puissant! 🎯

## Learning Objectives

- Définir des enums avec des données associées (c'est cool!)
- Utiliser des structs dans les enums
- Comprendre les différents styles de données
- Pattern matching avec données

## Core Explanation

### For Absolute Beginners - C'est Comme un Menu avec Options! 🍔

Imaginez un **menu McDo** 🍔 avec des options:
- **Enum simple** : Burger, Frites (juste le nom)
- **Enum avec données** : Burger(avec_fromage), Frites(taille) (nom + détails!)

C'est **exactement** comme les enums avec données fonctionnent! C'est **super puissant**!

## Schéma Visuel - Enums avec Données

```
┌─────────────────────────────────────────┐
│  🍔 ENUM AVEC DONNÉES 🍔               │
├─────────────────────────────────────────┤
│                                         │
│  enum Repas {                           │
│    Burger { fromage: bool },            │
│    Frites(taille),                      │
│    Boisson(String),                     │
│  }                                       │
│                                         │
│  Burger { fromage: true }               │
│  Frites("Grande")                       │
│  Boisson("Coca")                        │
│                                         │
│  Données associées! ✅                  │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "McDo" - Un enum avec données est comme un menu McDo: vous choisissez une option (Burger) et vous précisez les détails (avec fromage)!

## Code Examples

### Example 1: Enum avec Données Simples

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    match home {
        IpAddr::V4(a, b, c, d) => println!("IPv4: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(addr) => println!("IPv6: {}", addr),
    }
}
```

### Example 2: Enum avec Structs

```rust
enum Message {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
    ChangerCouleur(i32, i32, i32),
}

fn traiter_message(msg: Message) {
    match msg {
        Message::Quitter => println!("Quitter"),
        Message::Deplacer { x, y } => println!("Déplacer à ({}, {})", x, y),
        Message::Ecrire(text) => println!("Écrire: {}", text),
        Message::ChangerCouleur(r, g, b) => {
            println!("Couleur RGB({}, {}, {})", r, g, b);
        }
    }
}

fn main() {
    let msg1 = Message::Deplacer { x: 10, y: 20 };
    let msg2 = Message::Ecrire(String::from("Bonjour"));
    
    traiter_message(msg1);
    traiter_message(msg2);
}
```

### Example 3: Enum Complexe

```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { a: f64, b: f64, c: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { a, b, c } => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

fn main() {
    let circle = Shape::Circle { radius: 5.0 };
    println!("Aire du cercle: {:.2}", circle.area());
}
```

## Official Resources

- [@official Rust Book - Enums with Data](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#enum-values)
