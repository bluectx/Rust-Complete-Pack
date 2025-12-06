# Box<T> - Boîte Magique! 📦

## Learning Objectives

- Comprendre Box pour allouer sur le heap (c'est simple!)
- Utiliser Box pour les types récursifs
- Comprendre l'ownership avec Box
- Voir les cas d'usage

## Core Explanation

### For Absolute Beginners - C'est Comme une Boîte Postale! 📬

Imaginez une **boîte postale** 📬:
- **Box** = La boîte (sur votre bureau/stack)
- **Valeur** = Le colis (dans le casier/heap)
- Vous gardez juste la clé (Box) sur votre bureau!

C'est **exactement** comme Box fonctionne! C'est **super pratique**!

## Schéma Visuel - Box

```
┌─────────────────────────────────────────┐
│  📦 BOX = BOÎTE POSTALE 📦             │
├─────────────────────────────────────────┤
│                                         │
│  Stack (bureau):                         │
│  ┌─────┐                                │
│  │ Box │ ──┐                            │
│  └─────┘   │                            │
│            │ pointe vers                │
│  Heap (casier):                          │
│            ▼                            │
│  ┌─────────────┐                        │
│  │   Colis     │                        │
│  └─────────────┘                        │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Boîte" - Box est comme une boîte postale: vous gardez la clé (Box) sur votre bureau, le colis (valeur) est dans le casier (heap)!

## Code Examples

### Example 1: Box Basique

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    // b est libéré automatiquement à la fin du scope
    // La mémoire sur le heap est libérée
}
```

### Example 2: Type Récursif

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
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
    // Sans Box: sur la stack (peut causer stack overflow)
    // let large_array = [0u8; 1_000_000];
    
    // Avec Box: sur le heap
    let large_array = Box::new([0u8; 1_000_000]);
    println!("Taille: {} bytes", large_array.len());
}
```

### Example 4: Box pour Trait Objects

```rust
trait Draw {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Draw for Circle {
    fn draw(&self) {
        println!("Dessiner un cercle");
    }
}

fn main() {
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle),
        Box::new(Square),
    ];
    
    for shape in shapes {
        shape.draw();
    }
}
```

## Caractéristiques de Box

- **Ownership** : Box possède la valeur sur le heap
- **Deref** : Box implémente Deref (peut être utilisé comme &T)
- **Drop** : Libération automatique de la mémoire
- **Taille fixe** : Box lui-même a une taille fixe (pointeur)

## Official Resources

- [@official Rust Book - Box](https://doc.rust-lang.org/book/ch15-01-box.html)

## Security Notes

Box est sûr :
- Pas de use-after-free (ownership)
- Pas de double-free (un seul propriétaire)
- Libération automatique garantie
