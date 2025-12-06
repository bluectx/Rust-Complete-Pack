# Box<T> - Introduction

## Learning Objectives

- Comprendre Box (c'est simple!)
- Utiliser Box pour allouer sur le heap
- Utiliser Box pour les types récursifs
- Comprendre l'ownership avec Box

## Core Explanation

### For Absolute Beginners - C'est Comme une Boîte Postale! 📬

Imaginez une **boîte postale** 📬:
- **Box** = La boîte (sur votre bureau/stack)
- **Valeur** = Le colis (dans le casier/heap)
- Vous gardez juste la clé (Box) sur votre bureau!

C'est **exactement** comme Box fonctionne! C'est **super pratique**!

## Schéma Visuel - Box

```
┌─────────────────────────────────────────┐
│  📬 BOX = BOÎTE POSTALE 📬            │
├─────────────────────────────────────────┤
│                                         │
│  Stack (bureau):                        │
│  ┌─────┐                                │
│  │ Box │ ──┐                            │
│  └─────┘   │                            │
│            │ pointe vers                │
│  Heap (casier):                          │
│            ▼                            │
│  ┌─────────────┐                        │
│  │   Colis     │                        │
│  └─────────────┘                        │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Boîte Postale" - Box est comme une boîte postale: vous gardez la clé (Box) sur votre bureau, le colis (valeur) est dans le casier (heap)!

## Code Examples

### Example 1: Box Basique

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

## Official Resources

- [@official Rust Book - Box](https://doc.rust-lang.org/book/ch15-01-box.html)

