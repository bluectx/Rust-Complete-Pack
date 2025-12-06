# Send et Sync - Sécurité Thread! 🔒

## Learning Objectives

- Comprendre Send/Sync pour la sécurité (c'est essentiel!)
- Éviter les data races
- Utiliser Send/Sync correctement
- Voir les pièges

## Core Explanation

### For Absolute Beginners - C'est Comme un Passeport de Sécurité! 🛂

Imaginez un **passeport de sécurité** 🛂:
- **Send** = Vous pouvez voyager entre threads (sécurisé)
- **Sync** = Vous pouvez être partagé entre threads (sécurisé)
- Le compilateur vérifie automatiquement!

C'est **exactement** comme Send et Sync fonctionnent! C'est **super sûr**!

## Schéma Visuel - Send/Sync Sécurité

```
┌─────────────────────────────────────────┐
│  🛂 SEND/SYNC = PASSEPORT 🛂           │
├─────────────────────────────────────────┤
│                                         │
│  Send (transférer):                     │
│  Thread 1 → [Valeur] → Thread 2        │
│  ✅ Sécurisé!                           │
│                                         │
│  Sync (partager):                       │
│  Thread 1 ──┐                          │
│  Thread 2 ──┼─> [Valeur]               │
│  Thread 3 ──┘   ✅ Sécurisé!           │
│                                         │
│  Pas de data races! ✅                  │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Passeport" - Send/Sync sont comme un passeport de sécurité: ils garantissent que vous pouvez voyager/partager entre threads en toute sécurité!

## Code Examples

### Example 1: Send pour Sécurité

```rust
use std::thread;

// Types Send peuvent être transférés entre threads
fn safe_transfer() {
    let data = vec![1, 2, 3];  // Vec est Send
    thread::spawn(move || {
        println!("{:?}", data);  // OK
    });
}
```

### Example 2: Sync pour Partage

```rust
use std::sync::Arc;
use std::thread;

// Types Sync peuvent être partagés
fn safe_share() {
    let data = Arc::new(5);  // Arc est Send + Sync
    let data_clone = Arc::clone(&data);
    thread::spawn(move || {
        println!("{}", data_clone);  // OK
    });
}
```

### Example 3: Piège - Rc n'est pas Send

```rust
use std::rc::Rc;
use std::thread;

// ❌ ERREUR: Rc n'est pas Send
// let rc = Rc::new(5);
// thread::spawn(move || {
//     let _ = rc;  // ERREUR
// });
```

## Règles de Sécurité

- **Send** : Garantit pas de data race lors du transfert
- **Sync** : Garantit pas de data race lors du partage
- **Vérification à la compilation** : Empêche les erreurs

## Official Resources

- [@official Rust Book - Send and Sync](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)

## Security Notes

Send/Sync garantissent la sécurité thread :
- Empêchent les data races
- Vérifiés à la compilation
- Essentiels pour la sécurité concurrente

