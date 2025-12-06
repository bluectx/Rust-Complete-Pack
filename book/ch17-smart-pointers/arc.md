# Arc (Atomically Reference Counted)

## Learning Objectives

- Utiliser Arc pour le partage thread-safe
- Comprendre la différence avec Rc
- Utiliser Arc avec Mutex
- Voir les cas d'usage

## Code Examples

### Example 1: Arc Basique

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(5);
    
    // Cloner l'Arc (pas la donnée)
    let data_clone = Arc::clone(&data);
    
    let handle = thread::spawn(move || {
        println!("Valeur: {}", data_clone);
    });
    
    handle.join().unwrap();
    println!("Valeur originale: {}", data);
}
```

### Example 2: Arc avec Mutex

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

### Example 3: Partage de Données Complexes

```rust
use std::sync::Arc;
use std::thread;

struct Config {
    host: String,
    port: u16,
}

fn main() {
    let config = Arc::new(Config {
        host: "localhost".to_string(),
        port: 8080,
    });
    
    let config_clone = Arc::clone(&config);
    thread::spawn(move || {
        println!("Connexion à {}:{}", config_clone.host, config_clone.port);
    }).join().unwrap();
}
```

## Arc vs Rc

```
ARC
├── Thread-safe (Send + Sync)
├── Plus lent (opérations atomiques)
└── Utilisé pour multi-threading

RC
├── Single-threaded seulement
├── Plus rapide
└── Utilisé pour single-threaded
```

## Official Resources

- [@official Rust Book - Arc](https://doc.rust-lang.org/book/ch16-03-shared-state.html)

## Security Notes

Arc est thread-safe mais :
- Attention aux deadlocks avec Mutex
- Utiliser RwLock pour multiple readers
- Gérer les timeouts pour éviter les blocages
