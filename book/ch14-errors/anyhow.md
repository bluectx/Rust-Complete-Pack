# anyhow pour Gestion d'Erreurs - Super Facile! 🎯

## Learning Objectives

- Utiliser anyhow pour la gestion d'erreurs (c'est simple!)
- Créer des erreurs facilement
- Propager les erreurs avec ?
- Ajouter du contexte aux erreurs

## Core Explanation

### For Absolute Beginners - C'est Comme un Élastique! 🎈

Imaginez un **élastique** 🎈 qui s'étire:
- **anyhow** = Un élastique qui accepte n'importe quelle erreur
- Vous pouvez ajouter du contexte (étirer l'élastique)
- C'est **super flexible** et **super pratique**!

## Schéma Visuel - anyhow

```
┌─────────────────────────────────────────┐
│  🎈 ANYHOW = ÉLASTIQUE 🎈              │
├─────────────────────────────────────────┤
│                                         │
│  Erreur quelconque                      │
│         │                                │
│         ▼ anyhow::Result                 │
│  ┌─────────────┐                        │
│  │ Élastique   │ → Accepte tout!       │
│  └─────────────┘                        │
│         │                                │
│         ▼ with_context()                │
│  Erreur + Contexte                      │
│                                         │
│  Super flexible! ✅                     │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Élastique" - anyhow est comme un élastique: il accepte n'importe quelle erreur et peut s'étirer avec du contexte!

## Code Examples

### Example 1: anyhow Basique

```rust
use anyhow::{Context, Result};

fn lire_fichier(nom: &str) -> Result<String> {
    std::fs::read_to_string(nom)
        .with_context(|| format!("Impossible de lire {}", nom))
}

fn main() -> Result<()> {
    let contenu = lire_fichier("file.txt")?;
    println!("{}", contenu);
    Ok(())
}
```

### Example 2: Créer des Erreurs

```rust
use anyhow::{bail, Result};

fn process_number(n: i32) -> Result<i32> {
    if n < 0 {
        bail!("Nombre négatif: {}", n);
    }
    Ok(n * 2)
}

fn main() -> Result<()> {
    let result = process_number(-5)?;
    Ok(())
}
```

### Example 3: Contexte Multiple

```rust
use anyhow::{Context, Result};

fn process_data(filename: &str) -> Result<()> {
    let content = std::fs::read_to_string(filename)
        .with_context(|| format!("Lecture de {}", filename))?;
    
    let data: serde_json::Value = serde_json::from_str(&content)
        .with_context(|| format!("Parsing JSON de {}", filename))?;
    
    // Traitement
    Ok(())
}
```

## Avantages d'anyhow

- **Simple** : Pas besoin de définir des types d'erreur
- **Contexte** : Ajouter des informations aux erreurs
- **Propagation** : Utiliser ? facilement
- **Debug** : Erreurs détaillées avec chain d'erreurs

## Official Resources

- [anyhow crate](https://docs.rs/anyhow/)

## Security Notes

anyhow est sûr mais :
- Ne pas exposer les détails d'erreur aux utilisateurs
- Logger les erreurs complètes
- Masquer les informations sensibles
