# Tokio & Networking - Introduction

## Learning Objectives

- Comprendre Tokio (c'est puissant!)
- Créer des applications réseau async
- Utiliser TCP/UDP avec Tokio
- Gérer les connexions concurrentes

## Core Explanation

### For Absolute Beginners - C'est Comme un Restaurant avec Plusieurs Serveurs! 🍽️

Imaginez un **restaurant** 🍽️ avec plusieurs serveurs:
- **Tokio** = Le système qui gère plusieurs serveurs (tasks) en même temps
- Chaque serveur peut servir plusieurs clients (connexions) simultanément
- C'est **super efficace** pour les applications réseau!

C'est **exactement** comme Tokio fonctionne! C'est **super puissant**!

## Schéma Visuel - Tokio Networking

```
┌─────────────────────────────────────────┐
│  🍽️ TOKIO = RESTAURANT 🍽️             │
├─────────────────────────────────────────┤
│                                         │
│  Serveur Principal                       │
│  │                                      │
│  ├─> Task 1 (Serveur 1) → Client 1     │
│  ├─> Task 2 (Serveur 2) → Client 2     │
│  └─> Task 3 (Serveur 3) → Client 3     │
│                                         │
│  Tous servent EN MÊME TEMPS! ✅        │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Restaurant" - Tokio est comme un restaurant avec plusieurs serveurs: tous peuvent servir des clients simultanément, super efficace!

## Code Examples

### Example 1: Serveur TCP Basique

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
            socket.read(&mut buf).await.unwrap();
            socket.write_all(b"HTTP/1.1 200 OK\r\n\r\nHello!").await.unwrap();
        });
    }
}
```

## Official Resources

- [Tokio](https://tokio.rs/)

