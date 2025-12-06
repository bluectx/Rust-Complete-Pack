# Error Propagation Patterns - Relais d'Erreurs! 🎯

## Learning Objectives

- Comprendre les patterns de propagation d'erreurs (c'est pratique!)
- Utiliser ? efficacement
- Créer des chaînes d'erreurs
- Gérer les erreurs proprement

## Core Explanation

### For Absolute Beginners - C'est Comme un Relais! 🏃

Imaginez une **course de relais** 🏃:
- **?** = Passer le relais (erreur) au coureur suivant
- Si tout va bien, vous continuez
- Si erreur, vous passez le relais (erreur) en haut

C'est **exactement** comme la propagation d'erreurs fonctionne! C'est **super pratique**!

## Schéma Visuel - Propagation

```
┌─────────────────────────────────────────┐
│  🏃 PROPAGATION = RELAIS 🏃            │
├─────────────────────────────────────────┤
│                                         │
│  Fonction 1                             │
│    ↓ ? (erreur?)                        │
│  Fonction 2                             │
│    ↓ ? (erreur?)                        │
│  Fonction 3                             │
│                                         │
│  Si erreur → Remonte automatiquement!  │
│  Si OK → Continue!                     │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Relais" - La propagation d'erreurs est comme un relais: si erreur, elle remonte automatiquement, sinon vous continuez!

## Patterns

### Example 1: Propagation Simple

```rust
use std::fs::File;
use std::io::Read;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("file.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
```

### Example 2: Contexte avec anyhow

```rust
use anyhow::{Context, Result};

fn process() -> Result<()> {
    let content = std::fs::read_to_string("file.txt")
        .context("Lecture du fichier")?;
    
    let number: i32 = content.trim().parse()
        .context("Parsing du nombre")?;
    
    Ok(())
}
```

### Example 3: Types d'Erreur avec thiserror

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
}

fn process() -> Result<(), MyError> {
    // Propagation automatique
    let content = std::fs::read_to_string("file.txt")?;
    let number: i32 = content.trim().parse()?;
    Ok(())
}
```

## Official Resources

- [@official Rust Book - Error Propagation](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

