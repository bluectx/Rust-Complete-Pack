# Channels - Communication entre Threads! 📨

## Learning Objectives

- Utiliser channels pour message passing
- Comprendre mpsc (multiple producers, single consumer)
- Voir des exemples COOL

## Core Explanation

### For Absolute Beginners - C'est Comme un Tapis Roulant! 🎁

Imaginez que vous avez plusieurs ouvriers qui fabriquent des **colis**:
- Chaque ouvrier (thread) finit un colis
- Il l'envoie dans un **channel** (comme un tapis roulant!)
- Le réceptionniste (autre thread) récupère les colis

C'est **exactement** comme les channels fonctionnent! C'est **super pratique**!

## Schéma Visuel - Channels

```
┌─────────────────────────────────────────┐
│  📨 CHANNELS = TAPIS ROULANT 📨        │
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

**Mnémonique:** "Tapis Roulant" - Les threads envoient des messages (colis) sur un tapis roulant (channel), le réceptionniste (autre thread) les récupère dans l'ordre!

## Code Examples

### Example 1: Channel Basique (Super Facile!)

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // Créer un channel
    let (tx, rx) = mpsc::channel();
    
    // Créer un thread qui envoie
    thread::spawn(move || {
        let val = String::from("Message envoyé!");
        tx.send(val).unwrap();
    });
    
    // Recevoir dans le thread principal
    let received = rx.recv().unwrap();
    println!("Reçu: {}", received);  // "Message envoyé!"
}
```

## Official Resources

- [@official Rust Book - Channels](https://doc.rust-lang.org/book/ch16-02-message-passing.html)

