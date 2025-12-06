# Builder Pattern - Construction Flexible! 🎯

## Learning Objectives

- Comprendre le pattern Builder (c'est pratique!)
- Implémenter un builder en Rust
- Utiliser le builder pour des configurations complexes
- Comparer avec d'autres langages

## Core Explanation

### For Absolute Beginners - C'est Comme Construire une Maison! 🏠

Imaginez construire une **maison** 🏠:
- **Builder** = Vous construisez étape par étape
- Vous ajoutez les murs, puis le toit, puis les fenêtres
- À la fin, vous avez une maison complète!

C'est **exactement** comme le Builder pattern fonctionne! C'est **super flexible**!

## Schéma Visuel - Builder Pattern

```
┌─────────────────────────────────────────┐
│  🏠 BUILDER = CONSTRUCTION 🏠           │
├─────────────────────────────────────────┤
│                                         │
│  ConfigBuilder::new()                   │
│    .host("...")                         │
│    .port(8080)                          │
│    .timeout(60)                         │
│    .build()                             │
│         │                                │
│         ▼                                │
│  Config complète!                        │
│                                         │
│  Construction étape par étape! ✅       │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Construction" - Le Builder pattern construit un objet étape par étape, comme construire une maison!

## Code Examples

### Example 1: Builder Simple

```rust
struct Config {
    host: String,
    port: u16,
    timeout: u64,
}

struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<u64>,
}

impl ConfigBuilder {
    fn new() -> Self {
        ConfigBuilder {
            host: None,
            port: None,
            timeout: None,
        }
    }
    
    fn host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }
    
    fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    
    fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }
    
    fn build(self) -> Result<Config, String> {
        Ok(Config {
            host: self.host.ok_or("host is required")?,
            port: self.port.ok_or("port is required")?,
            timeout: self.timeout.unwrap_or(30),
        })
    }
}

fn main() {
    let config = ConfigBuilder::new()
        .host("localhost".to_string())
        .port(8080)
        .timeout(60)
        .build()
        .unwrap();
    
    println!("Config: {}:{}", config.host, config.port);
}
```

## Official Resources

- [@official Rust Design Patterns](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html)

