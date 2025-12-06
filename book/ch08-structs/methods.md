# Méthodes - Fonctions pour Structs! 🎯

## Learning Objectives

- Définir des méthodes avec impl (c'est facile!)
- Comprendre &self, &mut self, self
- Appeler des méthodes comme un pro

## Core Explanation

### For Absolute Beginners - C'est Comme des Actions! 🎬

Imaginez une **boîte** 📦 avec des **actions**:
- **Méthode** = Une action que la boîte peut faire
- **&self** = Regarder la boîte (pas la modifier)
- **&mut self** = Modifier la boîte
- **self** = Prendre la boîte (la consommer)

C'est **exactement** comme les méthodes fonctionnent! C'est **super pratique**!

## Schéma Visuel - Méthodes

```
┌─────────────────────────────────────────┐
│  🎬 MÉTHODES = ACTIONS 🎬              │
├─────────────────────────────────────────┤
│                                         │
│  struct Rectangle {                     │
│    width, height                        │
│  }                                       │
│                                         │
│  impl Rectangle {                       │
│    fn area(&self)      → Regarder       │
│    fn resize(&mut self) → Modifier      │
│    fn consume(self)    → Prendre        │
│  }                                       │
│                                         │
│  rect.area()  → Action sur la boîte!   │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Actions" - Les méthodes sont des actions que votre struct peut faire: regarder (&self), modifier (&mut self), ou prendre (self)!

## Code Examples

### Example 1: Méthodes Basiques

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("Aire: {}", rect1.area());
}
```

### Example 2: Méthodes Mutables

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
}

fn main() {
    let mut rect = Rectangle { width: 10, height: 20 };
    rect.double_size();
    println!("Nouvelle taille: {}x{}", rect.width, rect.height);
}
```

## Official Resources

- [@official Rust Book - Methods](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)

