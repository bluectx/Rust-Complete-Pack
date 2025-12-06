# HashMap - Dictionnaire Magique! 🎯

## Learning Objectives

- Utiliser HashMap pour stocker des paires clé-valeur (c'est simple!)
- Insérer et récupérer des valeurs
- Itérer sur un HashMap

## Core Explanation

### For Absolute Beginners - C'est Comme un Dictionnaire! 📖

Imaginez un **dictionnaire** 📖:
- **HashMap** = Un dictionnaire avec des mots (clés) et leurs définitions (valeurs)
- Vous cherchez un mot → Vous trouvez sa définition!
- C'est **super pratique** pour associer des choses!

C'est **exactement** comme HashMap fonctionne! C'est **super pratique**!

## Schéma Visuel - HashMap

```
┌─────────────────────────────────────────┐
│  📖 HASHMAP = DICTIONNAIRE 📖          │
├─────────────────────────────────────────┤
│                                         │
│  Clé      → Valeur                      │
│  "Blue"   → 10                          │
│  "Yellow" → 50                          │
│  "Red"    → 25                          │
│                                         │
│  Chercher "Blue" → Trouve 10! ✅       │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Dictionnaire" - HashMap est comme un dictionnaire: vous cherchez une clé (mot) et trouvez sa valeur (définition)!

## Code Examples

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
```

## Official Resources

- [@official Rust Book - HashMap](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)

