# Tokio Runtime - Autoroute Async! 🛣️

## Learning Objectives

- Utiliser Tokio comme runtime async (c'est puissant!)
- Comprendre les concepts de Tokio
- Créer des applications async
- Gérer les ressources

## Core Explanation

### For Absolute Beginners - C'est Comme une Autoroute! 🛣️

Imaginez une **autoroute** 🛣️:
- **Tokio** = L'autoroute qui gère le trafic async
- Plusieurs voitures (tâches) peuvent rouler en même temps
- C'est **super rapide** et **super efficace**!

C'est **exactement** comme Tokio fonctionne! C'est **super puissant**!

## Schéma Visuel - Tokio

```
┌─────────────────────────────────────────┐
│  🛣️ TOKIO = AUTOROUTE 🛣️              │
├─────────────────────────────────────────┤
│                                         │
│  Runtime Tokio                          │
│  ┌─────────────┐                        │
│  │ Tâche 1     │ → Voiture 1            │
│  │ Tâche 2     │ → Voiture 2            │
│  │ Tâche 3     │ → Voiture 3            │
│  └─────────────┘                        │
│                                         │
│  Toutes roulent en parallèle! ✅       │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Autoroute" - Tokio est comme une autoroute: plusieurs tâches (voitures) peuvent s'exécuter en parallèle, super rapide et efficace!

## Code Examples

### Example 1: Tokio Basique

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Avant");
    sleep(Duration::from_secs(1)).await;
    println!("Après");
}
```

### Example 2: Spawn de Tâches

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("Tâche spawnée");
    });
    
    handle.await.unwrap();
}
```

### Example 3: Serveur TCP

```rust
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    loop {
        let (mut socket, _) = listener.accept().await?;
        
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            if let Ok(n) = socket.read(&mut buf).await {
                let _ = socket.write_all(b"HTTP/1.1 200 OK\r\n\r\n").await;
            }
        });
    }
}
```

## Concepts Tokio

- **Runtime** : Gère l'exécution des futures
- **Spawn** : Démarrer une tâche concurrente
- **Await** : Attendre qu'une future se complète
- **Select** : Attendre plusieurs futures

## Official Resources

- [Tokio Documentation](https://tokio.rs/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

## Security Notes

Tokio est sûr mais :
- Gérer les timeouts
- Limiter les ressources
- Protéger contre les attaques (DDoS)
- Valider les inputs
