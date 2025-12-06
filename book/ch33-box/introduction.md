# Box<T> - Boîte Magique pour le Heap! 📦

## Learning Objectives

- Comprendre Box comme une boîte magique
- Utiliser Box pour allouer sur le heap
- Voir des exemples COOL avec types récursifs

## Core Explanation

### For Absolute Beginners - C'est Comme une Boîte Postale! 📬

Imaginez que vous avez un **colis** trop grand pour tenir sur votre bureau (stack). Vous le mettez dans une **boîte postale** (Box) et vous gardez juste la clé sur votre bureau! Le colis est dans le casier (heap), mais vous savez où il est grâce à la clé!

C'est **exactement** comme Box fonctionne! C'est **super facile**!

## Schéma Visuel - Box<T>

```
┌─────────────────────────────────────────┐
│  📦 BOX = BOÎTE POSTALE 📦              │
├─────────────────────────────────────────┤
│                                         │
│  Stack (bureau):                        │
│  ┌─────┐                                │
│  │ Box │ ──┐                            │
│  └─────┘   │                            │
│            │ pointe vers                │
│            │                            │
│  Heap (casier):                         │
│            │                            │
│            ▼                            │
│  ┌─────────────┐                        │
│  │   Colis     │                        │
│  └─────────────┘                        │
│                                         │
│  Box garde juste un pointeur!           │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Boîte Postale" - Vous gardez la clé (Box) sur votre bureau, le colis (valeur) est dans le casier (heap)!

## Code Examples

### Example 1: Box Basique (Super Facile!)

```rust
fn main() {
    // Créer un Box (boîte magique!)
    let b = Box::new(5);
    println!("Valeur dans la boîte: {}", b);
    
    // b est libéré automatiquement à la fin
    // La mémoire sur le heap est libérée automatiquement!
}
```

### Example 2: Type Récursif avec Box (Cool!)

```rust
// Liste chaînée (récursive!)
enum List {
    Cons(i32, Box<List>),  // Box permet la récursion!
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // Créer une liste [1, 2, 3]
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
    // Parcourir la liste
    match list {
        Cons(head, tail) => {
            println!("Tête: {}", head);
            // tail est un Box<List>
        }
        Nil => println!("Fin de la liste"),
    }
}
```

### Example 3: Box pour Grandes Structures

```rust
fn main() {
    // Sans Box: sur la stack (peut causer stack overflow!)
    // let large_array = [0u8; 1_000_000];  // ❌ Trop grand!
    
    // Avec Box: sur le heap (c'est OK!)
    let large_array = Box::new([0u8; 1_000_000]);
    println!("Taille: {} bytes", large_array.len());
    // ✅ Ça marche! Le colis est dans le casier!
}
```

## Official Resources

- [@official Rust Book - Box](https://doc.rust-lang.org/book/ch15-01-box.html)

