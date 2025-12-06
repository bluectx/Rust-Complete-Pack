# unwrap et expect - Attention! ⚠️

## Learning Objectives

- Comprendre unwrap() et expect() (c'est simple mais dangereux!)
- Savoir quand les utiliser (rarement!)
- Éviter les panics inutiles
- Gérer les erreurs proprement

## Core Explanation

### For Absolute Beginners - C'est Comme un Hacker! 🕵️

Imaginez un **hacker** 🕵️ qui force une porte:
- **unwrap/expect** = Forcer la porte (ça marche si la clé est bonne, sinon CRASH!)
- **match/?** = Vérifier d'abord si la clé fonctionne (plus sûr!)

C'est **exactement** comme unwrap fonctionne! C'est **super rapide** mais **super dangereux**!

## Schéma Visuel - unwrap vs match

```
┌─────────────────────────────────────────┐
│  ⚠️ UNWRAP = HACKER ⚠️                 │
├─────────────────────────────────────────┤
│                                         │
│  unwrap() (Dangereux!):                 │
│  Result → Forcer → Valeur ou CRASH!    │
│                                         │
│  match (Sûr!):                          │
│  Result → Vérifier → Valeur ou Erreur  │
│                                         │
│  ⚠️ Utiliser unwrap seulement dans     │
│     les tests ou prototypes!           │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Hacker" - unwrap force comme un hacker: ça marche si tout va bien, sinon ça plante!

## Code Examples

### Example 1: unwrap()

```rust
fn main() {
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;
    
    // unwrap: panique si None
    let value = x.unwrap();  // OK: retourne 5
    // let value2 = y.unwrap();  // PANIC!
    
    println!("Valeur: {}", value);
}
```

### Example 2: expect()

```rust
use std::fs::File;

fn main() {
    // expect: panique avec message personnalisé
    let file = File::open("hello.txt")
        .expect("Impossible d'ouvrir hello.txt");
    
    // Équivalent à:
    let file2 = match File::open("hello.txt") {
        Ok(f) => f,
        Err(e) => panic!("Impossible d'ouvrir hello.txt: {}", e),
    };
}
```

### Example 3: Quand Utiliser

```rust
// ❌ MAUVAIS: unwrap dans le code de production
fn process_file(filename: &str) {
    let file = File::open(filename).unwrap();  // Peut paniquer!
}

// ✅ BON: Gérer l'erreur
fn process_file(filename: &str) -> Result<(), std::io::Error> {
    let file = File::open(filename)?;
    // Traitement
    Ok(())
}

// ✅ ACCEPTABLE: unwrap dans les tests ou code de démo
#[test]
fn test_example() {
    let result = some_function().unwrap();  // OK dans les tests
    assert_eq!(result, expected);
}
```

## Règles d'Utilisation

- **unwrap/expect** : Seulement dans :
  - Tests
  - Prototypes
  - Code où on est sûr que ça ne peut pas échouer
- **Production** : Toujours utiliser `?` ou `match`

## Official Resources

- [@official Rust Book - unwrap](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

## Quand Utiliser unwrap/expect

### ✅ Acceptable
- Tests unitaires
- Prototypes rapides
- Code où on est 100% sûr que ça ne peut pas échouer
- Exemples pédagogiques

### ❌ Jamais
- Code de production
- Code qui traite des données utilisateur
- Code réseau/fichiers
- Code critique

## Security Notes

Les panics peuvent être exploitées :
- Éviter unwrap dans le code de production
- Toujours gérer les erreurs
- Utiliser Result<T, E> pour les erreurs récupérables
- Préférer `?` ou `match` pour la propagation d'erreurs
