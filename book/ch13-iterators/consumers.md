# Consumers d'Itérateurs - Consommer le Résultat! 🍽️

## Learning Objectives

- Utiliser les consumers (collect, sum, count, etc.) (c'est pratique!)
- Comprendre quand consommer un iterator
- Utiliser for_each

## Core Explanation

### For Absolute Beginners - C'est Comme Consommer un Repas! 🍽️

Imaginez que vous **consommez** 🍽️ un repas:
- **collect** = Mettre dans une assiette (collection)
- **sum** = Additionner tout
- **count** = Compter les éléments
- C'est **super pratique** pour obtenir des résultats!

C'est **exactement** comme les consumers fonctionnent! C'est **super simple**!

## Schéma Visuel - Consumers

```
┌─────────────────────────────────────────┐
│  🍽️ CONSUMERS = CONSOMMER 🍽️          │
├─────────────────────────────────────────┤
│                                         │
│  Iterator [1, 2, 3, 4, 5]              │
│         │                                │
│         ├─> collect() → Vec             │
│         ├─> sum() → 15                 │
│         ├─> count() → 5                 │
│         └─> for_each() → Exécuter      │
│                                         │
│  Consommer le résultat! ✅              │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Consommer" - Les consumers consomment l'itérateur pour obtenir un résultat: collect met dans une collection, sum additionne, count compte!

## Code Examples

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // collect: collecte dans une collection
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    
    // sum: somme
    let sum: i32 = v.iter().sum();
    
    // count: compter
    let count = v.iter().count();
    
    // for_each: exécuter pour chaque élément
    v.iter().for_each(|x| println!("{}", x));
}
```

## Official Resources

- [@official Rust Book - Iterator Consumers](https://doc.rust-lang.org/book/ch13-02-iterators.html#consuming-adapters)

