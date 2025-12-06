# Design Patterns Rust - Recettes Éprouvées! 🎯

## Learning Objectives

- Comprendre les patterns idiomatiques Rust (c'est pratique!)
- Utiliser les patterns courants
- Adapter les patterns classiques à Rust
- Éviter les anti-patterns

## Core Explanation

### For Absolute Beginners - C'est Comme des Recettes! 📖

Les design patterns sont comme des **recettes** 📖 éprouvées pour résoudre des problèmes courants. En Rust, certains patterns sont particulièrement utiles et idiomatiques.

**Analogie :**
- **Pattern** = Une recette de cuisine testée et approuvée
- Vous suivez la recette → Ça marche à tous les coups!
- C'est **super pratique** et **super fiable**!

## Schéma Visuel - Patterns

```
┌─────────────────────────────────────────┐
│  📖 PATTERNS = RECETTES 📖            │
├─────────────────────────────────────────┤
│                                         │
│  Builder Pattern                        │
│  └─> Construire étape par étape        │
│                                         │
│  Newtype Pattern                        │
│  └─> Distinguer types similaires        │
│                                         │
│  RAII Pattern                           │
│  └─> Libération automatique             │
│                                         │
│  Tous testés et approuvés! ✅          │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Recette" - Les patterns sont comme des recettes de cuisine: testées, approuvées, toujours efficaces!

## Key Vocabulary

| Term | Definition |
|------|-----------|
| Pattern | Solution réutilisable à un problème commun |
| Builder | Pattern pour construire des objets complexes |
| Newtype | Pattern pour créer des types wrapper |
| RAII | Resource Acquisition Is Initialization |

## Patterns Courants en Rust

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
    
    fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    
    fn build(self) -> Result<Config, String> {
        Ok(Config {
            host: self.host.ok_or("host required")?,
            port: self.port.ok_or("port required")?,
        })
    }
}
```

### Example 2: Newtype Pattern

```rust
struct Meters(f64);
struct Kilometers(f64);

fn calculer_distance(m: Meters) -> f64 {
    m.0
}

fn main() {
    let distance = Meters(1000.0);
    // Impossible de passer Kilometers par erreur
    calculer_distance(distance);
}
```

### Example 3: RAII Pattern

```rust
struct Guard {
    // Ressource qui sera libérée automatiquement
}

impl Drop for Guard {
    fn drop(&mut self) {
        // Libération automatique
        println!("Ressource libérée");
    }
}
```

## Patterns Rust Spécifiques

- **Builder** : Construction flexible
- **Newtype** : Type safety
- **Type-state** : États à la compilation
- **RAII** : Gestion automatique des ressources

## Official Resources

- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)
- [@official Rust Book - Patterns](https://doc.rust-lang.org/book/ch17-00-oop.html)

## Security Notes

Les patterns Rust aident à la sécurité :
- Newtype évite les erreurs de type
- RAII empêche les fuites de ressources
- Builder valide les configurations
