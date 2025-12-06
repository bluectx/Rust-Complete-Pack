# Adapters d'Itérateurs - Transformer en Chaîne! ⛓️

## Learning Objectives

- Utiliser les adapters (map, filter, take, etc.) (c'est puissant!)
- Chaîner les adapters
- Comprendre l'évaluation paresseuse

## Core Explanation

### For Absolute Beginners - C'est Comme une Chaîne de Transformation! ⛓️

Imaginez une **chaîne de transformation** ⛓️:
- **Adapters** = Transformations que vous enchaînez
- map transforme, filter filtre, take prend les premiers
- C'est **super puissant** et **super élégant**!

C'est **exactement** comme les adapters fonctionnent! C'est **super pratique**!

## Schéma Visuel - Adapters

```
┌─────────────────────────────────────────┐
│  ⛓️ ADAPTERS = CHAÎNE TRANSFORMATION ⛓️│
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
│         ▼ take(2)                        │
│  [6, 8]                                 │
│                                         │
│  Chaîne élégante! ✅                    │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Chaîne Transformation" - Les adapters créent une chaîne de transformation: map transforme, filter filtre, take prend!

## Code Examples

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    let result: Vec<i32> = v.iter()
        .map(|x| x * 2)
        .filter(|x| x > &5)
        .take(3)
        .collect();
    
    println!("{:?}", result);  // [6, 8, 10]
}
```

## Official Resources

- [@official Rust Book - Iterator Adapters](https://doc.rust-lang.org/book/ch13-02-iterators.html#iterator-adapters)

