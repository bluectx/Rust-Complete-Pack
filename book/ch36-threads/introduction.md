# Threads - Programmation Parallèle Facile! 🚀

## Learning Objectives

- Créer des threads comme un pro
- Comprendre Send et Sync
- Voir des exemples COOL avec parallélisme

## Core Explanation

### For Absolute Beginners - C'est Comme Cuisiner Plusieurs Plats en Même Temps! 👨‍🍳

Imaginez que vous cuisinez plusieurs **plats** en même temps:
- Chaque **thread** = Un cuisinier qui fait un plat
- Tous les cuisiniers travaillent **en parallèle** (plus rapide!)
- À la fin, tous les plats sont prêts!

C'est **exactement** comme les threads fonctionnent! C'est **super rapide**!

## Schéma Visuel - Threads

```
┌─────────────────────────────────────────┐
│  🚀 THREADS = CUISINIERS PARALLÈLES 🚀 │
├─────────────────────────────────────────┤
│                                         │
│  Thread 1 → Plat A                      │
│  Thread 2 → Plat B                      │
│  Thread 3 → Plat C                      │
│                                         │
│  Tous cuisinent EN MÊME TEMPS!          │
│  Résultat: 3 plats plus vite! ⚡       │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Cuisiniers Parallèles" - Plusieurs cuisiniers travaillent en même temps sur différents plats, résultat: tout est prêt plus vite!

## Code Examples

### Example 1: Créer des Threads (Super Facile!)

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Créer un thread
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Code principal continue en parallèle
    for i in 1..=5 {
        println!("Main: {}", i);
        thread::sleep(Duration::from_millis(100));
    }
    
    // Attendre que le thread finisse
    handle.join().unwrap();
}
```

## Official Resources

- [@official Rust Book - Threads](https://doc.rust-lang.org/book/ch16-01-threads.html)

