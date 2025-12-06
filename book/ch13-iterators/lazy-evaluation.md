# Évaluation Paresseuse - Faire Quand Nécessaire! 😴

## Learning Objectives

- Comprendre que les iterators sont lazy (c'est intelligent!)
- Voir comment l'évaluation se fait
- Optimiser avec les iterators

## Core Explanation

### For Absolute Beginners - C'est Comme Faire Quand Nécessaire! 😴

Imaginez que vous êtes **paresseux** 😴:
- **Lazy** = Ne faire le travail que quand c'est vraiment nécessaire
- Les iterators ne calculent rien jusqu'à ce que vous consommiez
- C'est **super intelligent** et **super efficace**!

C'est **exactement** comme l'évaluation paresseuse fonctionne! C'est **super optimisé**!

## Schéma Visuel - Lazy Evaluation

```
┌─────────────────────────────────────────┐
│  😴 LAZY = FAIRE QUAND NÉCESSAIRE 😴  │
├─────────────────────────────────────────┤
│                                         │
│  iterator.map(|x| x * 2)                │
│         │                                │
│         └─> Rien ne se passe encore!   │
│                                         │
│  iterator.collect()                      │
│         │                                │
│         ▼ Maintenant ça calcule!       │
│  [2, 4, 6, 8, 10]                       │
│                                         │
│  Calcul seulement quand nécessaire! ✅  │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Faire Quand Nécessaire" - L'évaluation paresseuse fait le travail seulement quand nécessaire, comme être paresseux mais intelligent!

## Code Examples

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // Rien ne se passe jusqu'à collect()
    let iterator = v.iter().map(|x| {
        println!("Traitement: {}", x);
        x * 2
    });
    
    // Maintenant l'évaluation se fait
    let result: Vec<i32> = iterator.collect();
}
```

## Official Resources

- [@official Rust Book - Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)

