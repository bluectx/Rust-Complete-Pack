# Propagation d'Erreurs - C'est Super Facile! 🎯

## Learning Objectives

- Utiliser ? pour propager les erreurs (c'est simple!)
- Comprendre la différence avec match
- Gérer les erreurs proprement
- Convertir entre types d'erreurs

## Core Explanation

### For Absolute Beginners - C'est Comme Passer le Relais! 🏃

Imaginez une **course de relais** 🏃:
- **?** = Passer le relais (erreur) au coureur suivant
- Si tout va bien, vous continuez
- Si erreur, vous passez le relais (erreur) en haut

C'est **exactement** comme ? fonctionne! C'est **super pratique**!

## Schéma Visuel - Propagation avec ?

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
│  Si OK → Continue!                      │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Relais" - L'opérateur ? passe l'erreur comme un relais: si erreur, elle remonte automatiquement, sinon vous continuez!

## Code Examples

### Example 1: Opérateur ?

```rust
use std::fs::File;
use std::io::Read;

fn lire_fichier(nom: &str) -> Result<String, std::io::Error> {
    let mut fichier = File::open(nom)?;  // ? propage l'erreur
    let mut contenu = String::new();
    fichier.read_to_string(&mut contenu)?;  // ? propage l'erreur
    Ok(contenu)
}

fn main() {
    match lire_fichier("hello.txt") {
        Ok(contenu) => println!("Contenu: {}", contenu),
        Err(e) => println!("Erreur: {}", e),
    }
}
```

### Example 2: ? vs match

```rust
// Avec match (verbose)
fn avec_match() -> Result<String, std::io::Error> {
    let mut file = match File::open("file.txt") {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    // ...
}

// Avec ? (concis)
fn avec_question() -> Result<String, std::io::Error> {
    let mut file = File::open("file.txt")?;
    // ...
    Ok(String::new())
}
```

### Example 3: Conversion d'Erreurs

```rust
use std::fs::File;
use std::io::Read;

fn lire_et_parser(nom: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut file = File::open(nom)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let number: i32 = content.trim().parse()?;
    Ok(number)
}
```

### Example 4: Contexte avec anyhow

```rust
use anyhow::{Context, Result};

fn process() -> Result<()> {
    let content = std::fs::read_to_string("file.txt")
        .with_context(|| "Impossible de lire le fichier")?;
    
    let number: i32 = content.trim().parse()
        .with_context(|| "Impossible de parser le nombre")?;
    
    Ok(())
}
```

## Règles de ?

- **?** peut être utilisé dans les fonctions qui retournent Result
- **?** convertit automatiquement les erreurs compatibles
- **?** retourne immédiatement en cas d'erreur
- **?** fonctionne avec Option aussi (retourne None)

## Official Resources

- [@official Rust Book - Error Propagation](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#propagating-errors)

## Security Notes

La propagation d'erreurs est sûre :
- Pas de perte d'information
- Chaîne d'erreurs complète
- Pas de silent failures
