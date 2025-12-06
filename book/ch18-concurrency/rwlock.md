# RwLock - Lectures Multiples! 📖

## Learning Objectives

- Utiliser RwLock pour multiple readers (c'est optimisé!)
- Comprendre la différence avec Mutex
- Optimiser les lectures concurrentes

## Core Explanation

### For Absolute Beginners - C'est Comme une Bibliothèque! 📚

Imaginez une **bibliothèque** 📚:
- **Mutex** = Une seule personne peut lire (verrou exclusif)
- **RwLock** = Plusieurs personnes peuvent lire en même temps (verrou partagé pour lecture, exclusif pour écriture)!

C'est **exactement** comme RwLock fonctionne! C'est **super optimisé**!

## Schéma Visuel - RwLock vs Mutex

```
┌─────────────────────────────────────────┐
│  📚 RWLOCK = BIBLIOTHÈQUE 📚          │
├─────────────────────────────────────────┤
│                                         │
│  Mutex:                                 │
│  ┌─────┐                                │
│  │ 1   │ → Une seule personne           │
│  └─────┘                                │
│                                         │
│  RwLock:                                │
│  ┌─────┐ ┌─────┐ ┌─────┐              │
│  │ R1  │ │ R2  │ │ R3  │ → Plusieurs  │
│  └─────┘ └─────┘ └─────┘   lecteurs!  │
│                                         │
│  Optimisé pour lectures! ✅            │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Bibliothèque" - RwLock est comme une bibliothèque: plusieurs personnes peuvent lire en même temps, mais une seule peut écrire à la fois!

## Code Examples

```rust
use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(5);
    
    // Multiple readers
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
    }
    
    // Un seul writer
    {
        let mut w = lock.write().unwrap();
        *w += 1;
    }
}
```

## Official Resources

- [@official Rust Book - Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

