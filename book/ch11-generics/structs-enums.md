# Structs et Enums Génériques - Types Flexibles! 🎯

## Learning Objectives

- Définir des structs génériques (c'est puissant!)
- Définir des enums génériques
- Utiliser plusieurs type parameters

## Core Explanation

### For Absolute Beginners - C'est Comme des Boîtes Universelles! 📦

Imaginez des **boîtes universelles** 📦:
- **Struct/Enum générique** = Une boîte qui peut contenir différents types
- Vous mettez des i32 ou des f64 → La même boîte!
- C'est **super pratique** pour créer des types flexibles!

C'est **exactement** comme les structs/enums génériques fonctionnent! C'est **super puissant**!

## Schéma Visuel - Types Génériques

```
┌─────────────────────────────────────────┐
│  📦 TYPES GÉNÉRIQUES = BOÎTES 📦      │
├─────────────────────────────────────────┤
│                                         │
│  struct Point<T> {                      │
│      x: T,                              │
│      y: T,                              │
│  }                                      │
│                                         │
│  Point<i32> → (5, 10)                   │
│  Point<f64> → (1.0, 4.0)                │
│                                         │
│  Une boîte pour tous! ✅                │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Boîtes Universelles" - Les types génériques sont comme des boîtes universelles: elles peuvent contenir différents types, créant des structures flexibles!

## Code Examples

```rust
struct Point<T> {
    x: T,
    y: T,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let point_int = Point { x: 5, y: 10 };
    let point_float = Point { x: 1.0, y: 4.0 };
}
```

## Official Resources

- [@official Rust Book - Generic Structs](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-struct-definitions)

