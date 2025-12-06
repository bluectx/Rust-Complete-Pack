# Arc & Mutex - Partage Thread-Safe! 🔒

## Learning Objectives

- Comprendre Arc comme Rc thread-safe
- Utiliser Mutex pour synchronisation
- Voir des exemples COOL avec threads

## Core Explanation

### For Absolute Beginners - C'est Comme Partager un Cahier entre Élèves! 📓

Imaginez que plusieurs élèves veulent écrire dans le même **cahier** 📓 en même temps:
- **Arc** = Compteur atomique thread-safe (plusieurs threads peuvent partager!)
- **Mutex** = Verrou (une personne écrit à la fois, les autres attendent!)

C'est **super sûr** et **super pratique** pour le multi-threading!

## Schéma Visuel - Arc & Mutex

```
┌─────────────────────────────────────────┐
│  🔒 ARC + MUTEX = PARTAGE SÛR 🔒       │
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

**Mnémonique:** "Compteur Atomique" - Un compteur thread-safe qui permet à plusieurs threads de partager une valeur en toute sécurité, avec un verrou (Mutex) pour éviter les conflits!

## Code Examples

### Example 1: Arc avec Mutex (Super Facile!)

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Créer un compteur partagé
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    // Créer 10 threads
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;  // Incrémenter (sûr grâce à Mutex!)
        });
        handles.push(handle);
    }
    
    // Attendre tous les threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Résultat: {}", *counter.lock().unwrap());  // 10
}
```

## Official Resources

- [@official Rust Book - Arc](https://doc.rust-lang.org/book/ch16-03-shared-state.html)

