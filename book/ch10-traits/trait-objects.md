# Trait Objects (dyn) - Polymorphisme Dynamique! 🎭

## Learning Objectives

- Comprendre les trait objects (c'est puissant!)
- Utiliser dyn Trait
- Comprendre le dynamic dispatch

## Core Explanation

### For Absolute Beginners - C'est Comme un Masque! 🎭

Imaginez un **masque** 🎭:
- **Trait object** = Un masque qui cache le type réel
- Vous pouvez traiter différents types de la même façon
- C'est **super puissant** pour le polymorphisme!

C'est **exactement** comme les trait objects fonctionnent! C'est **super flexible**!

## Schéma Visuel - Trait Objects

```
┌─────────────────────────────────────────┐
│  🎭 TRAIT OBJECT = MASQUE 🎭          │
├─────────────────────────────────────────┤
│                                         │
│  Circle ──┐                             │
│  Square ──┼─> dyn Draw (masque)        │
│  Triangle─┘                             │
│                                         │
│  Tous traités de la même façon! ✅     │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Masque" - Un trait object est comme un masque: il cache le type réel et permet de traiter différents types de la même façon!

## Code Examples

```rust
trait Draw {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Draw for Circle {
    fn draw(&self) {
        println!("Dessiner un cercle");
    }
}

fn main() {
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle),
        Box::new(Square),
    ];
    
    for shape in shapes {
        shape.draw();
    }
}
```

## Official Resources

- [@official Rust Book - Trait Objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)

