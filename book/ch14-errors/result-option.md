# Result et Option - Les Types Magiques! 🎯

## Learning Objectives

- Comprendre les types `Result<T, E>` et `Option<T>` (c'est simple!)
- Utiliser `match` pour gérer les erreurs
- Comprendre `unwrap()` et `expect()`
- Utiliser `?` pour la propagation d'erreurs
- Distinguer erreurs récupérables et non-récupérables

## Core Explanation

### For Absolute Beginners - C'est Comme Oui/Non ou Succès/Erreur! ✅❌

Rust utilise deux types magiques pour gérer les valeurs optionnelles et les erreurs:
- **Option<T>** : "Y a-t-il une valeur?" → Some(valeur) ou None
- **Result<T, E>** : "Ça a marché?" → Ok(valeur) ou Err(erreur)

C'est **exactement** comme ça fonctionne! C'est **super sûr**!

## Schéma Visuel - Option vs Result

```
┌─────────────────────────────────────────┐
│  ✅❌ OPTION vs RESULT ✅❌             │
├─────────────────────────────────────────┤
│                                         │
│  Option<T>:                             │
│  ├─> Some(valeur)  → Oui! ✅           │
│  └─> None          → Non! ❌          │
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

## Key Vocabulary

| Term | Definition |
|------|-----------|
| Result<T, E> | Type pour opérations qui peuvent échouer (Ok ou Err) |
| Option<T> | Type pour valeurs optionnelles (Some ou None) |
| unwrap() | Extrait la valeur ou panique |
| expect() | Extrait avec message d'erreur personnalisé |
| ? | Opérateur de propagation d'erreur |
| Panic | Erreur non-récupérable qui arrête le programme |

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

fn ouvrir_fichier(nom: &str) -> Result<File, std::io::Error> {
    File::open(nom)
}

fn main() {
    match ouvrir_fichier("fichier.txt") {
        Ok(fichier) => println!("Fichier ouvert avec succès"),
        Err(e) => println!("Erreur: {}", e),
    }
}
```

### Example 3: Propagation avec ?

```rust
use std::fs::File;
use std::io::Read;

fn lire_fichier(nom: &str) -> Result<String, std::io::Error> {
    let mut fichier = File::open(nom)?;  // ? propage l'erreur
    let mut contenu = String::new();
    fichier.read_to_string(&mut contenu)?;
    Ok(contenu)
}

fn main() {
    match lire_fichier("fichier.txt") {
        Ok(contenu) => println!("Contenu: {}", contenu),
        Err(e) => println!("Erreur: {}", e),
    }
}
```

## Comparaisons avec Autres Langages

### En C (Gestion d'Erreurs Manuelle)

```c
// C: Codes de retour, propice aux erreurs
FILE* f = fopen("file.txt", "r");
if (f == NULL) {
    // Gérer l'erreur
    return -1;
}
// Facile d'oublier de vérifier
```

### En Rust (Système de Types)

```rust
// Rust: Forcé par le type system
let f = File::open("file.txt")?;  // Doit gérer l'erreur
// Impossible d'oublier
```

## Official Resources

- [@official Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

## Security Notes

La gestion d'erreurs explicite empêche :
- **Silent failures** : Les erreurs doivent être gérées
- **Unchecked errors** : Le type system force la gestion
- **Resource leaks** : Les erreurs sont propagées proprement

