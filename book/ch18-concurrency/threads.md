# Threads - Programmation Parallèle! 🚀

## Learning Objectives

- Créer des threads (c'est facile!)
- Utiliser join() pour attendre
- Comprendre move dans les closures de threads

## Core Explanation

### For Absolute Beginners - C'est Comme des Cuisiniers! 👨‍🍳

Imaginez plusieurs **cuisiniers** 👨‍🍳 qui travaillent en même temps:
- Chaque **thread** = Un cuisinier
- Tous travaillent **en parallèle** (plus rapide!)
- À la fin, tous les plats sont prêts!

C'est **exactement** comme les threads fonctionnent! C'est **super rapide**!

## Schéma Visuel - Threads

```
┌─────────────────────────────────────────┐
│  👨‍🍳 THREADS = CUISINIERS 👨‍🍳        │
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

**Mnémonique:** "Cuisiniers" - Les threads sont comme des cuisiniers qui travaillent en parallèle: plusieurs tâches en même temps, résultat plus rapide!

## Code Examples

### Example 1: Créer un Thread

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Bonjour depuis un thread!");
    });
    
    handle.join().unwrap();
    println!("Thread terminé");
}
```

### Example 2: move dans Threads

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    
    let handle = thread::spawn(move || {
        println!("Vecteur: {:?}", v);
    });
    
    handle.join().unwrap();
}
```

## Official Resources

- [@official Rust Book - Threads](https://doc.rust-lang.org/book/ch16-01-threads.html)

