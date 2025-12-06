# Rc (Reference Counting) - Partage Intelligent! 🤝

## Learning Objectives

- Utiliser Rc pour le partage de données (c'est facile!)
- Comprendre le reference counting
- Gérer les cycles (attention!)
- Voir les limitations

## Core Explanation

### For Absolute Beginners - C'est Comme Partager! 🤝

Imaginez que vous **partagez** 🤝 quelque chose avec des amis:
- **Rc** = Un compteur qui dit "Combien de personnes partagent?"
- Quand le compteur arrive à 0, la valeur est libérée!
- Plusieurs personnes peuvent regarder en même temps!

C'est **exactement** comme Rc fonctionne! C'est **super intelligent**!

## Schéma Visuel - Rc

```
┌─────────────────────────────────────────┐
│  🤝 RC = PARTAGE 🤝                    │
├─────────────────────────────────────────┤
│                                         │
│  Valeur (ex: 5)                         │
│  │                                      │
│  ├─> Rc::new(5)                        │
│  │   Compteur: 1                       │
│  │                                      │
│  ├─> Rc::clone(&rc)                    │
│  │   Compteur: 2                       │
│  │                                      │
│  └─> Rc::clone(&rc)                    │
│      Compteur: 3                       │
│                                         │
│  Quand compteur = 0 → Valeur libérée!  │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Partage" - Rc permet de partager une valeur entre plusieurs propriétaires, avec un compteur qui suit combien de personnes partagent!

## Code Examples

### Example 1: Rc Basique

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(5);
    let b = Rc::clone(&a);  // Partage, pas copie
    let c = Rc::clone(&a);
    
    println!("Compteur: {}", Rc::strong_count(&a));  // 3
    
    // a, b, c pointent tous vers la même valeur
    println!("a = {}, b = {}, c = {}", a, b, c);
}
```

### Example 2: Partage de Données

```rust
use std::rc::Rc;

struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

fn main() {
    let node1 = Rc::new(Node {
        value: 1,
        next: None,
    });
    
    let node2 = Rc::new(Node {
        value: 2,
        next: Some(Rc::clone(&node1)),  // Partage node1
    });
    
    // node1 et node2.next pointent vers la même valeur
}
```

### Example 3: Limitations

```rust
use std::rc::Rc;

// Rc n'est PAS thread-safe
// let rc = Rc::new(5);
// thread::spawn(move || {
//     let _ = rc;  // ERREUR: Rc n'implémente pas Send
// });

// Pour le partage entre threads, utiliser Arc
```

## Rc vs Arc

```
RC (Reference Counting)
├── Single-threaded seulement
├── Plus rapide (pas de synchronisation)
└── Pas thread-safe

ARC (Atomically Reference Counted)
├── Multi-threaded
├── Plus lent (synchronisation atomique)
└── Thread-safe
```

## Official Resources

- [@official Rust Book - Rc](https://doc.rust-lang.org/book/ch15-04-rc.html)

## Avantages de Rc

- **Partage** : Plusieurs propriétaires pour une valeur
- **Automatique** : Libération quand compteur = 0
- **Efficace** : Pas de copie, juste partage
- **Single-threaded** : Plus rapide que Arc

## Limitations

- **Pas thread-safe** : Ne peut pas être partagé entre threads
- **Cycles possibles** : Peut créer des memory leaks
- **Overhead** : Compteur ajoute un petit coût

## Security Notes

Rc peut créer des cycles (memory leaks) :
- Utiliser Weak<T> pour casser les cycles
- Attention aux références circulaires
- Préférer Arc pour le multi-threading
- Toujours vérifier les cycles dans les structures complexes
