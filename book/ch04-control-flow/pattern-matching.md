# Pattern Matching (Introduction) - C'est Super Puissant! 🎯

## Learning Objectives

- Comprendre match comme expression (c'est exhaustif!)
- Utiliser match avec différents patterns
- Comprendre l'exhaustiveness
- Utiliser if let et while let

## Core Explanation

### For Absolute Beginners - C'est Comme un Menu avec Choix! 🍔

Imaginez un **menu McDo** 🍔:
- **match** = Vous devez choisir parmi TOUTES les options
- Le compilateur vérifie que vous n'oubliez rien!
- C'est **super sûr** et **super pratique**!

C'est **exactement** comme pattern matching fonctionne! C'est **super puissant**!

## Schéma Visuel - Pattern Matching

```
┌─────────────────────────────────────────┐
│  🍔 MATCH = MENU EXHAUSTIF 🍔          │
├─────────────────────────────────────────┤
│                                         │
│  match nombre {                         │
│      1 => "Un",                         │
│      2 => "Deux",                       │
│      3 => "Trois",                      │
│      _ => "Autre",                      │
│  }                                      │
│                                         │
│  Tous les cas couverts! ✅              │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "McDo" - match est comme un menu McDo: vous devez traiter TOUTES les options, le compilateur vérifie que rien n'est oublié!

## Code Examples

### Example 1: Match Basique

```rust
fn main() {
    let nombre = 3;
    
    match nombre {
        1 => println!("Un"),
        2 => println!("Deux"),
        3 => println!("Trois"),
        _ => println!("Autre"),  // Cas par défaut
    }
}
```

### Example 2: Match avec Retour

```rust
fn valeur_en_mots(nombre: u32) -> &'static str {
    match nombre {
        1 => "un",
        2 => "deux",
        3 => "trois",
        _ => "beaucoup",
    }
}

fn main() {
    println!("{}", valeur_en_mots(2));
}
```

### Example 3: if let

```rust
fn main() {
    let option = Some(5);
    
    // Au lieu de match complet
    if let Some(valeur) = option {
        println!("Valeur: {}", valeur);
    }
    
    // Équivalent à:
    match option {
        Some(valeur) => println!("Valeur: {}", valeur),
        None => {},
    }
}
```

## Official Resources

- [@official Rust Book - match](https://doc.rust-lang.org/book/ch06-02-match.html)

