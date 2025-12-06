# Arc et Mutex - Partage Thread-Safe! 🔒

## Learning Objectives

- Utiliser Arc<Mutex<T>> pour le partage thread-safe (c'est sûr!)
- Protéger les données partagées
- Éviter les deadlocks

## Core Explanation

### For Absolute Beginners - C'est Comme un Cahier Partagé! 📓

Imaginez plusieurs élèves qui veulent écrire dans le même **cahier** 📓:
- **Arc** = Compteur thread-safe (plusieurs threads peuvent partager!)
- **Mutex** = Verrou (une personne écrit à la fois, les autres attendent!)

C'est **exactement** comme Arc et Mutex fonctionnent! C'est **super sûr**!

## Schéma Visuel - Arc & Mutex

```
┌─────────────────────────────────────────┐
│  🔒 ARC + MUTEX = CAHIER PARTAGÉ 🔒   │
├─────────────────────────────────────────┤
│                                         │
│  Cahier 📓 (valeur partagée)            │
│  │                                      │
│  ├─> Arc::new(Mutex::new(cahier))      │
│  │   Thread 1 ──┐                      │
│  │   Thread 2 ──┼─> Verrou (Mutex)     │
│  │   Thread 3 ──┘   Une personne       │
│  │                  écrit à la fois!    │
│                                         │
│  Pas de data races! ✅                  │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Cahier" - Arc permet de partager un cahier entre plusieurs threads, Mutex assure qu'une seule personne écrit à la fois!

## Code Examples

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Résultat: {}", *counter.lock().unwrap());
}
```

## Official Resources

- [@official Rust Book - Shared State](https://doc.rust-lang.org/book/ch16-03-shared-state.html)

