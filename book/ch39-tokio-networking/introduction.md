# Tokio & Networking - Serveurs Web Faciles! 🌐

## Learning Objectives

- Utiliser Tokio pour async networking
- Créer des serveurs HTTP simples
- Voir des exemples COOL

## Core Explanation

### For Absolute Beginners - C'est Comme un Restaurant Efficace! 🍽️

Imaginez un **restaurant** qui sert plusieurs clients:
- **Tokio** = Le système qui gère plusieurs clients en même temps
- Chaque client (connexion) est servi **en parallèle**
- Pas besoin d'attendre qu'un client finisse pour servir le suivant!

C'est **exactement** comme Tokio fonctionne! C'est **super rapide**!

## Schéma Visuel - Tokio

```
┌─────────────────────────────────────────┐
│  🌐 TOKIO = RESTAURANT EFFICACE 🌐     │
├─────────────────────────────────────────┤
│                                         │
│  Client 1 → Plat A                      │
│  Client 2 → Plat B                      │
│  Client 3 → Plat C                      │
│                                         │
│  Tous servis EN MÊME TEMPS!            │
│  (Pas d'attente!)                      │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Restaurant Efficace" - Plusieurs clients sont servis simultanément, pas besoin d'attendre qu'un client finisse pour servir le suivant!

## Code Examples

### Example 1: Serveur TCP Simple (Super Facile!)

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
                let _ = socket.write_all(b"HTTP/1.1 200 OK\r\n\r\nHello from Tokio!").await;
            }
        });
    }
}
```

## Official Resources

- [Tokio Documentation](https://tokio.rs/)

