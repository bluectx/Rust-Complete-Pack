# thiserror pour Types d'Erreurs - Type-Safe! 🎯

## Learning Objectives

- Créer des types d'erreurs personnalisés (c'est facile!)
- Utiliser derive avec thiserror
- Implémenter Error trait facilement
- Chaîner les erreurs

## Core Explanation

### For Absolute Beginners - C'est Comme des Étiquettes! 🏷️

Imaginez des **étiquettes** 🏷️ sur des boîtes d'erreurs:
- **thiserror** = Créer des étiquettes personnalisées
- Chaque type d'erreur = une étiquette différente
- Le compilateur vérifie que vous utilisez la bonne étiquette!

C'est **exactement** comme thiserror fonctionne! C'est **super sûr**!

## Schéma Visuel - thiserror

```
┌─────────────────────────────────────────┐
│  🏷️ THISERROR = ÉTIQUETTES 🏷️         │
├─────────────────────────────────────────┤
│                                         │
│  enum MyError {                         │
│    ┌─────────────┐                      │
│    │ IoError     │ → Étiquette 1       │
│    ├─────────────┤                      │
│    │ ParseError  │ → Étiquette 2       │
│    ├─────────────┤                      │
│    │ CustomError │ → Étiquette 3       │
│    └─────────────┘                      │
│  }                                       │
│                                         │
│  Type-safe! ✅                          │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Étiquettes" - thiserror crée des étiquettes personnalisées pour vos erreurs, chaque type d'erreur a sa propre étiquette!

## Code Examples

### Example 1: Erreur Personnalisée

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("Erreur IO: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Valeur invalide: {0}")]
    InvalidValue(String),
    
    #[error("Parsing error: {0}")]
    Parse(#[from] std::num::ParseIntError),
}

fn process_file(filename: &str) -> Result<(), MyError> {
    let content = std::fs::read_to_string(filename)?;  // ? convertit en MyError
    let number: i32 = content.trim().parse()?;
    
    if number < 0 {
        return Err(MyError::InvalidValue(number.to_string()));
    }
    
    Ok(())
}
```

### Example 2: Erreur avec Contexte

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum ConfigError {
    #[error("Fichier de config introuvable: {path}")]
    FileNotFound { path: String },
    
    #[error("Clé manquante: {key}")]
    MissingKey { key: String },
}

fn load_config() -> Result<(), ConfigError> {
    Err(ConfigError::FileNotFound {
        path: "config.toml".to_string(),
    })
}
```

### Example 3: Combiner avec anyhow

```rust
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("Erreur: {0}")]
    Custom(String),
}

fn function() -> Result<(), MyError> {
    Err(MyError::Custom("Erreur".to_string()))
}

fn main() -> Result<()> {
    function()?;  // Convertit automatiquement
    Ok(())
}
```

## Avantages de thiserror

- **Type-safe** : Types d'erreur explicites
- **Debug** : Implémentation automatique de Debug
- **Display** : Messages d'erreur personnalisés
- **From** : Conversion automatique avec #[from]

## Official Resources

- [thiserror crate](https://docs.rs/thiserror/)

## Security Notes

- Ne pas exposer les détails internes dans les messages d'erreur
- Logger les erreurs complètes pour le debugging
- Masquer les chemins de fichiers sensibles
