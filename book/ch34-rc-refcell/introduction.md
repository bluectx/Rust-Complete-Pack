# Rc & RefCell - Partage Intelligent! 🤝

## Learning Objectives

- Comprendre Rc comme un compteur de partage
- Utiliser RefCell pour interior mutability
- Voir des exemples COOL

## Core Explanation

### For Absolute Beginners - C'est Comme Partager un Livre! 📚

Imaginez que vous avez un **livre** 📚 et plusieurs amis veulent le lire:
- **Rc** = Compteur qui dit "Combien de personnes partagent ce livre?"
- Quand le compteur arrive à 0, le livre est libéré (plus personne ne le veut!)

C'est **exactement** comme Rc fonctionne! C'est **super intelligent**!

## Schéma Visuel - Rc & RefCell

```
┌─────────────────────────────────────────┐
│  🤝 RC = COMPTEUR DE PARTAGE 🤝        │
├─────────────────────────────────────────┤
│                                         │
│  Livre 📚 (valeur)                      │
│  │                                      │
│  ├─> Rc::new(livre)                    │
│  │   Compteur: 1                       │
│  │                                      │
│  ├─> Rc::clone(&rc)                    │
│  │   Compteur: 2                       │
│  │                                      │
│  └─> Rc::clone(&rc)                    │
│      Compteur: 3                       │
│                                         │
│  Quand compteur = 0 → Livre libéré!    │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Compteur de Partage" - Chaque personne qui partage le livre incrémente le compteur, quand personne ne le veut plus (compteur = 0), le livre est libéré!

## Code Examples

### Example 1: Rc Basique (Super Facile!)

```rust
use std::rc::Rc;

fn main() {
    // Créer un Rc (compteur initial: 1)
    let a = Rc::new(5);
    
    // Cloner (partager, pas copier!)
    let b = Rc::clone(&a);  // Compteur: 2
    let c = Rc::clone(&a);  // Compteur: 3
    
    println!("Compteur: {}", Rc::strong_count(&a));  // 3
    
    // a, b, c pointent tous vers la même valeur
    println!("a = {}, b = {}, c = {}", a, b, c);
    // Tous affichent: 5
}
```

### Example 2: RefCell - Interior Mutability (Cool!)

```rust
use std::cell::RefCell;

fn main() {
    // RefCell permet mutation même si immuable
    let data = RefCell::new(5);
    
    {
        let mut borrow = data.borrow_mut();
        *borrow += 1;  // Modifier!
    }  // borrow libéré ici
    
    let read = data.borrow();
    println!("Valeur: {}", *read);  // 6
}
```

## Official Resources

- [@official Rust Book - Rc](https://doc.rust-lang.org/book/ch15-04-rc.html)

