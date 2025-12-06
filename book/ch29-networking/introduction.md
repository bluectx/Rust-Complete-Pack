# Networking en Rust

## Learning Objectives

- Créer des applications réseau
- Utiliser TCP/UDP
- Gérer les connexions
- Utiliser les frameworks web

## Key Vocabulary

| Term | Definition |
|------|-----------|
| TCP | Transmission Control Protocol (fiable) |
| UDP | User Datagram Protocol (rapide) |
| HTTP | HyperText Transfer Protocol |
| Socket | Point de communication réseau |

## Core Explanation

### For Absolute Beginners - C'est Comme Envoyer des Lettres! 📮

Imaginez envoyer des **lettres** 📮 entre ordinateurs:
- **Networking** = Envoyer des lettres (données) entre ordinateurs
- **TCP** = Lettre recommandée (fiable, arrive toujours)
- **UDP** = Carte postale (rapide, peut se perdre)

C'est **exactement** comme le networking fonctionne! C'est **super pratique**!

## Schéma Visuel - Networking

```
┌─────────────────────────────────────────┐
│  📮 NETWORKING = LETTRES 📮            │
├─────────────────────────────────────────┤
│                                         │
│  Ordinateur 1                            │
│         │                                │
│         ▼ Envoie lettre                 │
│  ┌─────────────┐                        │
│  │   Réseau    │                        │
│  └─────────────┘                        │
│         │                                │
│         ▼ Reçoit lettre                 │
│  Ordinateur 2                            │
│                                         │
│  Communication réussie! ✅              │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Lettres" - Le networking est comme envoyer des lettres: vous envoyez des données (lettres) entre ordinateurs via le réseau!

Le networking, c'est comme envoyer des lettres entre ordinateurs. Rust permet de créer des applications qui communiquent sur le réseau, comme les sites web ou les serveurs de jeux.

**Types de communication :**
- **TCP** : Fiable, comme une lettre recommandée
- **UDP** : Rapide, comme une carte postale (peut se perdre)

## Crates Principales

### Example 1: TCP avec std

```rust
use std::net::TcpListener;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;
        stream.write(b"HTTP/1.1 200 OK\r\n\r\nHello!")?;
    }
    
    Ok(())
}
```

### Example 2: HTTP avec reqwest

```rust
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://httpbin.org/get")
        .await?
        .text()
        .await?;
    
    println!("{}", response);
    Ok(())
}
```

### Example 3: Serveur Web avec Axum

```rust
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

## Crates Recommandées

- **tokio** : Runtime async
- **reqwest** : Client HTTP
- **axum** : Framework web moderne
- **hyper** : HTTP bas niveau
- **serde** : Sérialisation JSON

## Official Resources

- [Tokio](https://tokio.rs/)
- [Axum](https://github.com/tokio-rs/axum)
- [@official Rust Book - Networking](https://doc.rust-lang.org/book/)

## Security Notes

- Toujours valider les inputs réseau
- Utiliser HTTPS en production
- Limiter les timeouts
- Gérer les erreurs de connexion
- Protéger contre les attaques (DDoS, injection)
