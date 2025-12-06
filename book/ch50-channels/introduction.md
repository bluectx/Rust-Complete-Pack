# Channels - Introduction

## Learning Objectives

- Comprendre les channels (c'est simple!)
- Utiliser des channels pour la communication
- Comprendre mpsc (multi-producer, single-consumer)
- Envoyer et recevoir des messages

## Core Explanation

### For Absolute Beginners - C'est Comme un Tapis Roulant! 🎁

Imaginez un **tapis roulant** 🎁:
- **Threads producteurs** = Ouvriers qui mettent des colis sur le tapis
- **Thread consommateur** = Réceptionniste qui récupère les colis
- **Channel** = Le tapis roulant qui transporte les messages!

C'est **exactement** comme les channels fonctionnent! C'est **super pratique**!

## Schéma Visuel - Channels

```
┌─────────────────────────────────────────┐
│  🎁 CHANNELS = TAPIS ROULANT 🎁        │
├─────────────────────────────────────────┤
│                                         │
│  Thread 1 → [Colis 1] ──┐              │
│  Thread 2 → [Colis 2] ──┼─> Channel    │
│  Thread 3 → [Colis 3] ──┘    (tapis)   │
│                                         │
│                    ▼                    │
│              Réceptionniste récupère     │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Tapis" - Les channels sont comme un tapis roulant: les threads envoient des messages (colis) sur le tapis, le réceptionniste (autre thread) les récupère dans l'ordre!

## Code Examples

### Example 1: Channel Basique

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        tx.send("hello").unwrap();
    });
    
    let received = rx.recv().unwrap();
    println!("Reçu: {}", received);
}
```

## Official Resources

- [@official Rust Book - Channels](https://doc.rust-lang.org/book/ch16-02-message-passing.html)

