# Ergonomie API Rust - APIs Agréables! 🎯

## Learning Objectives

- Créer des APIs ergonomiques (c'est important!)
- Utiliser les patterns idiomatiques
- Améliorer l'expérience utilisateur
- Voir les exemples

## Core Explanation

### For Absolute Beginners - C'est Comme une Interface Intuitive! 🎨

Imaginez une **interface intuitive** 🎨:
- **API ergonomique** = Facile à utiliser, agréable
- Les utilisateurs comprennent immédiatement
- C'est **super important** pour la satisfaction!

C'est **exactement** comme créer des APIs ergonomiques! C'est **super pratique**!

## Schéma Visuel - Ergonomie

```
┌─────────────────────────────────────────┐
│  🎨 ERGONOMIE = INTERFACE INTUITIVE 🎨 │
├─────────────────────────────────────────┤
│                                         │
│  API Ergonomique:                       │
│  - Facile à utiliser ✅                │
│  - Noms clairs ✅                      │
│  - Composable ✅                       │
│  - Type-safe ✅                        │
│                                         │
│  Utilisateurs heureux! 😊              │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Interface Intuitive" - Une API ergonomique est comme une interface intuitive: facile à utiliser, agréable, les utilisateurs comprennent immédiatement!

## Patterns Ergonomiques

### Example 1: Builder Pattern

```rust
struct Config {
    host: String,
    port: u16,
}

struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
}

impl ConfigBuilder {
    fn new() -> Self {
        ConfigBuilder { host: None, port: None }
    }
    
    fn host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }
    
    fn build(self) -> Result<Config, String> {
        Ok(Config {
            host: self.host.ok_or("host required")?,
            port: self.port.ok_or("port required")?,
        })
    }
}

// Usage ergonomique
let config = ConfigBuilder::new()
    .host("localhost".to_string())
    .port(8080)
    .build()?;
```

### Example 2: Default Trait

```rust
#[derive(Default)]
struct Config {
    host: String,
    port: u16,
}

// Usage
let config = Config {
    host: "localhost".to_string(),
    ..Default::default()
};
```

### Example 3: From/Into

```rust
struct Meters(f64);

impl From<f64> for Meters {
    fn from(value: f64) -> Self {
        Meters(value)
    }
}

// Usage ergonomique
let distance: Meters = 1000.0.into();
```

## Principes

- **Composabilité** : APIs qui se combinent bien
- **Type safety** : Types qui empêchent les erreurs
- **Clarté** : Noms explicites
- **Flexibilité** : Plusieurs façons d'utiliser

## Official Resources

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

