# Memory Model - Comprendre la Mémoire! 🧠

## Learning Objectives

- Comprendre stack vs heap
- Voir comment Rust gère la mémoire
- Voir des exemples COOL

## Core Explanation

### For Absolute Beginners - C'est Comme Organiser Votre Chambre! 🏠

Imaginez que vous organisez votre **chambre**:
- **Stack** = Votre bureau (petit, rapide, organisé)
- **Heap** = Votre armoire (grand, flexible, moins rapide)

Rust gère tout **automatiquement**! C'est **super intelligent**!

## Schéma Visuel - Stack vs Heap

```
┌─────────────────────────────────────────┐
│  🧠 MÉMOIRE = BUREAU + ARMOIRE 🧠      │
├─────────────────────────────────────────┤
│                                         │
│  Stack (Bureau):                        │
│  ┌───┐ ┌───┐ ┌───┐                     │
│  │ 5 │ │ 3 │ │ 7 │  (petit, rapide)    │
│  └───┘ └───┘ └───┘                     │
│                                         │
│  Heap (Armoire):                        │
│  ┌─────────────┐                        │
│  │ String      │  (grand, flexible)     │
│  │ "Hello"     │                        │
│  └─────────────┘                        │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique Stack:** "Bureau Stable" - Petit, rapide, organisé, comme un bureau où tout est à portée de main.

**Mnémonique Heap:** "Armoire Flexible" - Grand, flexible, moins rapide, comme une armoire où vous rangez les grandes choses.

## Code Examples

### Example 1: Stack vs Heap

```rust
fn main() {
    // Stack (petit, rapide)
    let x = 5;  // Sur la stack
    
    // Heap (grand, flexible)
    let s = String::from("Hello, World!");  // Sur le heap
}
```

## Official Resources

- [@official Rust Book - Memory](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)

