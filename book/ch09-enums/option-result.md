# Option et Result - C'est Super Important! 🎯

## Learning Objectives

- Comprendre Option<T> pour valeurs optionnelles (c'est simple!)
- Comprendre Result<T, E> pour gestion d'erreurs
- Utiliser match avec Option et Result
- Utiliser les méthodes pratiques (unwrap, expect, etc.)

## Core Explanation

### For Absolute Beginners - C'est Comme Oui/Non ou Succès/Erreur! ✅❌

Imaginez deux situations:
- **Option** : "Y a-t-il une valeur?" → Oui (Some) ou Non (None)
- **Result** : "Ça a marché?" → Succès (Ok) ou Erreur (Err)

C'est **exactement** comme Option et Result fonctionnent! C'est **super pratique**!

## Schéma Visuel - Option vs Result

```
┌─────────────────────────────────────────┐
│  ✅❌ OPTION vs RESULT ✅❌             │
├─────────────────────────────────────────┤
│                                         │
│  Option<T>:                             │
│  ├─> Some(valeur)  → Oui, il y a! ✅   │
│  └─> None          → Non, rien ❌      │
│                                         │
│  Result<T, E>:                          │
│  ├─> Ok(valeur)    → Succès! ✅        │
│  └─> Err(erreur)   → Erreur! ❌        │
│                                         │
│  Type-safe! ✅                          │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique Option:** "Oui/Non" - Option vous dit s'il y a une valeur (Some) ou pas (None)!

**Mnémonique Result:** "Succès/Erreur" - Result vous dit si ça a marché (Ok) ou pas (Err)!

## Code Examples

### Example 1: Option<T>

```rust
fn trouver_index(v: &[i32], valeur: i32) -> Option<usize> {
    for (i, &item) in v.iter().enumerate() {
        if item == valeur {
            return Some(i);
        }
    }
    None
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    
    match trouver_index(&vec, 3) {
        Some(index) => println!("Trouvé à l'index {}", index),
        None => println!("Non trouvé"),
    }
}
```

### Example 2: Result<T, E>

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    
    match f {
        Ok(file) => println!("Fichier ouvert avec succès"),
        Err(error) => println!("Erreur: {}", error),
    }
}
```

## Official Resources

- [@official Rust Book - Option](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values)
- [@official Rust Book - Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

