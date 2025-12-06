# Performance des Iterators - Aussi Rapides! ⚡

## Learning Objectives

- Comprendre que les iterators sont optimisés (c'est rapide!)
- Comparer avec les boucles
- Utiliser les iterators efficacement

## Core Explanation

### For Absolute Beginners - C'est Aussi Rapide que les Boucles! ⚡

Imaginez que vous avez deux façons de faire:
- **Boucles** = Méthode traditionnelle (rapide)
- **Iterators** = Méthode moderne (aussi rapide grâce aux optimisations!)
- Le compilateur optimise les iterators pour être aussi rapides!
- C'est **super pratique** et **super rapide**!

C'est **exactement** comme la performance des iterators fonctionne! C'est **super optimisé**!

## Schéma Visuel - Performance

```
┌─────────────────────────────────────────┐
│  ⚡ PERFORMANCE = AUSSI RAPIDE ⚡      │
├─────────────────────────────────────────┤
│                                         │
│  Boucle:                                │
│  for x in v { sum += x }                │
│  ⏱️ Temps: 5ms                          │
│                                         │
│  Iterator:                              │
│  v.iter().sum()                         │
│  ⏱️ Temps: 5ms (même vitesse!)          │
│                                         │
│  Zero-cost abstractions! ✅            │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Aussi Rapide" - Les iterators sont aussi rapides que les boucles grâce aux optimisations du compilateur, zero-cost abstractions!

## Code Examples

```rust
// Les iterators sont souvent aussi rapides que les boucles
// grâce à l'optimisation du compilateur

fn avec_boucle(v: &[i32]) -> i32 {
    let mut sum = 0;
    for &x in v {
        sum += x;
    }
    sum
}

fn avec_iterator(v: &[i32]) -> i32 {
    v.iter().sum()
}
```

## Official Resources

- [@official Rust Book - Performance](https://doc.rust-lang.org/book/ch13-04-performance.html)

