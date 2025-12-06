# Send et Sync - Sécurité Thread! 🔒

## Learning Objectives

- Comprendre les traits Send et Sync (c'est important!)
- Savoir quels types sont Send/Sync
- Utiliser Send/Sync dans le code
- Résoudre les problèmes de Send/Sync

## Core Explanation

### For Absolute Beginners - C'est Comme un Passeport! 🛂

Imaginez un **passeport** 🛂:
- **Send** = Vous pouvez voyager (transférer entre threads)
- **Sync** = Vous pouvez partager (plusieurs threads peuvent regarder)

C'est **exactement** comme Send et Sync fonctionnent! C'est **super sûr**!

## Schéma Visuel - Send vs Sync

```
┌─────────────────────────────────────────┐
│  🛂 SEND vs SYNC 🛂                    │
├─────────────────────────────────────────┤
│                                         │
│  Send (transférer):                     │
│  Thread 1 → [Valeur] → Thread 2        │
│  "Vous pouvez voyager!"                 │
│                                         │
│  Sync (partager):                       │
│  Thread 1 ──┐                          │
│  Thread 2 ──┼─> [Valeur]               │
│  Thread 3 ──┘   "Plusieurs regardent!" │
│                                         │
│  Type-safe! ✅                          │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique Send:** "Passeport" - Send permet de transférer une valeur entre threads, comme un passeport permet de voyager!

**Mnémonique Sync:** "Partage" - Sync permet de partager une valeur entre plusieurs threads via références!

## Code Examples

### Example 1: Send

```rust
use std::thread;

// Send: peut être transféré entre threads
fn main() {
    let x = 5;  // i32 est Send
    thread::spawn(move || {
        println!("{}", x);  // OK: i32 est Send
    }).join().unwrap();
}
```

### Example 2: Sync

```rust
use std::sync::Arc;
use std::thread;

// Sync: peut être partagé entre threads via références
fn main() {
    let data = Arc::new(5);  // Arc est Send + Sync
    let data_clone = Arc::clone(&data);
    
    thread::spawn(move || {
        println!("{}", data_clone);  // OK: Arc est Sync
    }).join().unwrap();
}
```

### Example 3: Types Non-Send

```rust
use std::rc::Rc;
use std::thread;

// Rc n'est PAS Send
fn main() {
    let rc = Rc::new(5);
    // thread::spawn(move || {
    //     let _ = rc;  // ERREUR: Rc n'implémente pas Send
    // });
}
```

## Règles

```
SEND
├── Type peut être transféré entre threads
├── Ownership peut être déplacé
└── Exemples: i32, String, Vec<T>, Arc<T>

SYNC
├── Type peut être partagé entre threads
├── &T est Send si T est Sync
└── Exemples: i32, String, Arc<T>, Mutex<T>

NON-SEND/SYNC
├── Rc<T> (single-threaded)
├── RefCell<T> (single-threaded)
└── *const T, *mut T (unsafe)
```

## Official Resources

- [@official Rust Book - Send and Sync](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)

## Security Notes

Send/Sync garantissent la sécurité thread :
- Empêchent les data races
- Vérifiés à la compilation
- Essentiels pour la sécurité concurrente
