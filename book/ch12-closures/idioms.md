# Idiomes avec Closures - Code Élégant! ✨

## Learning Objectives

- Utiliser les closures avec iterators (c'est élégant!)
- Comprendre map, filter, fold
- Écrire du code idiomatique Rust

## Core Explanation

### For Absolute Beginners - C'est Comme une Chaîne de Traitement! ⛓️

Imaginez une **chaîne de traitement** ⛓️:
- **map** = Transformer chaque élément
- **filter** = Filtrer les éléments
- **fold** = Réduire à une valeur
- C'est **super élégant** et **super puissant**!

C'est **exactement** comme les idiomes avec closures fonctionnent! C'est **super pratique**!

## Schéma Visuel - Idiomes

```
┌─────────────────────────────────────────┐
│  ⛓️ IDIOMES = CHAÎNE TRAITEMENT ⛓️    │
├─────────────────────────────────────────┤
│                                         │
│  [1, 2, 3, 4, 5]                        │
│         │                                │
│         ▼ map(|x| x * 2)                │
│  [2, 4, 6, 8, 10]                       │
│         │                                │
│         ▼ filter(|x| x > 5)             │
│  [6, 8, 10]                             │
│         │                                │
│         ▼ fold(0, |acc, x| acc + x)    │
│  24                                     │
│                                         │
│  Chaîne élégante! ✅                    │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Chaîne Traitement" - Les idiomes avec closures créent une chaîne de traitement: map transforme, filter filtre, fold réduit!

## Code Examples

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // map: transformer
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    
    // filter: filtrer
    let evens: Vec<&i32> = numbers.iter().filter(|x| x % 2 == 0).collect();
    
    // fold: réduire
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
}
```

## Official Resources

- [@official Rust Book - Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)

