# Création d'Itérateurs - Créer des Vues! 👁️

## Learning Objectives

- Créer des itérateurs (c'est simple!)
- Comprendre Iterator trait
- Utiliser iter(), into_iter(), iter_mut()

## Core Explanation

### For Absolute Beginners - C'est Comme Créer des Vues! 👁️

Imaginez que vous **créez des vues** 👁️ sur une collection:
- **iter()** = Vue qui emprunte (ne consomme pas)
- **into_iter()** = Vue qui prend possession (consomme)
- **iter_mut()** = Vue mutable (peut modifier)
- C'est **super simple** et **super pratique**!

C'est **exactement** comme créer des itérateurs fonctionne! C'est **super logique**!

## Schéma Visuel - Création

```
┌─────────────────────────────────────────┐
│  👁️ ITÉRATEURS = VUES 👁️              │
├─────────────────────────────────────────┤
│                                         │
│  Vec [1, 2, 3]                          │
│         │                                │
│         ├─> iter() → Vue empruntée     │
│         ├─> into_iter() → Prend possession│
│         └─> iter_mut() → Vue mutable    │
│                                         │
│  Différentes vues! ✅                   │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Vues" - Créer un itérateur, c'est créer une vue sur une collection: empruntée, possession, ou mutable!

## Code Examples

### Example 1: Créer un Itérateur

```rust
fn main() {
    let v1 = vec![1, 2, 3];
    
    // iter() - emprunte
    let v1_iter = v1.iter();
    
    // into_iter() - prend ownership
    let v1_into = v1.into_iter();
    
    // iter_mut() - emprunte mutablement
    let mut v2 = vec![1, 2, 3];
    let v2_iter = v2.iter_mut();
}
```

## Official Resources

- [@official Rust Book - Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)

