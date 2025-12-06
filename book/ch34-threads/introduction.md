# Threads - Introduction

## Learning Objectives

- Comprendre les threads (c'est simple!)
- Créer des threads
- Utiliser join() pour attendre
- Comprendre move dans les closures de threads

## Core Explanation

### For Absolute Beginners - C'est Comme des Cuisiniers! 🧑‍🍳

Imaginez que vous avez une grande cuisine et plusieurs **cuisiniers** (vos threads). Chaque cuisinier peut travailler sur une tâche différente en même temps (préparer un plat, couper des légumes, faire la vaisselle). Cela permet de faire beaucoup plus de travail **en parallèle** et plus rapidement! C'est **super efficace** pour les tâches lourdes!

## Schéma Visuel - Threads

```
┌─────────────────────────────────────────┐
│  🧑‍🍳 THREADS = CUISINIERS PARALLÈLES 🧑‍🍳 │
├─────────────────────────────────────────┤
│                                         │
│  Tâche Principale (Chef)                │
│  │                                      │
│  ├─> Thread 1 (Cuisinier 1) → Préparer Entrée │
│  ├─> Thread 2 (Cuisinier 2) → Cuire Plat Principal │
│  └─> Thread 3 (Cuisinier 3) → Faire Dessert │
│                                         │
│  Tous travaillent EN MÊME TEMPS! ✅     │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Cuisiniers Parallèles" - Les threads permettent d'exécuter plusieurs tâches en même temps, comme des cuisiniers dans une cuisine!

## Code Examples

### Example 1: Créer un Thread

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Bonjour depuis le thread!");
    });
    
    handle.join().unwrap();
}
```

## Official Resources

- [@official Rust Book - Threads](https://doc.rust-lang.org/book/ch16-01-threads.html)

