# Booléens et Caractères - C'est Super Simple! 🎯

## Learning Objectives

- Comprendre le type bool (true/false) (c'est simple!)
- Comprendre le type char (caractères Unicode)
- Utiliser les opérateurs logiques
- Manipuler les caractères Unicode

## Core Explanation

### For Absolute Beginners - C'est Comme Oui/Non et Lettres! ✅❌

Imaginez:
- **bool** = Répondre "Oui" (true) ou "Non" (false)
- **char** = Une lettre ou un caractère (Unicode)

C'est **exactement** comme bool et char fonctionnent! C'est **super simple**!

## Schéma Visuel - Bool & Char

```
┌─────────────────────────────────────────┐
│  ✅❌ BOOL & CHAR = OUI/NON, LETTRES ✅❌ │
├─────────────────────────────────────────┤
│                                         │
│  bool:                                  │
│  ├─> true  → Oui ✅                    │
│  └─> false → Non ❌                    │
│                                         │
│  char:                                  │
│  ├─> 'A' → Lettre                      │
│  ├─> '🦀' → Emoji                      │
│  └─> '中' → Unicode                    │
│                                         │
│  Simple et puissant! ✅                │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Oui/Non, Lettres" - bool = oui ou non, char = lettres et caractères Unicode!

## Key Vocabulary

| Term | Definition |
|------|-----------|
| bool | Type booléen (true ou false) |
| char | Type caractère Unicode (4 bytes) |
| Opérateur logique | && (ET), \|\| (OU), ! (NON) |
| Unicode | Standard d'encodage de caractères |

## Code Examples

### Example 1: Type bool

```rust
fn main() {
    let vrai = true;
    let faux = false;
    
    // Opérateurs logiques
    let et = vrai && faux;      // false
    let ou = vrai || faux;      // true
    let non = !vrai;            // false
    
    println!("ET: {}, OU: {}, NON: {}", et, ou, non);
    
    // Comparaisons retournent bool
    let est_egal = 5 == 5;     // true
    let est_different = 5 != 3; // true
    let est_superieur = 10 > 5; // true
    
    println!("Égal: {}, Différent: {}, Supérieur: {}", 
             est_egal, est_different, est_superieur);
}
```

### Example 2: Type char

```rust
fn main() {
    // char en Rust est Unicode (4 bytes), pas ASCII
    let lettre = 'A';
    let emoji = '🦀';
    let accent = 'é';
    let chinois = '中';
    
    println!("Lettre: {}, Emoji: {}, Accent: {}, Chinois: {}", 
             lettre, emoji, accent, chinois);
    
    // Méthodes utiles
    println!("'A' est alphabétique: {}", lettre.is_alphabetic());
    println!("'5' est numérique: {}", '5'.is_numeric());
    println!("'a' en majuscule: {}", 'a'.to_uppercase());
}
```

### Example 3: Utilisation Pratique

```rust
fn verifier_age(age: u32) -> bool {
    age >= 18
}

fn main() {
    let age = 20;
    let peut_voter = verifier_age(age);
    
    if peut_voter {
        println!("Vous pouvez voter!");
    } else {
        println!("Vous êtes trop jeune pour voter.");
    }
    
    // Utilisation avec match
    let statut = match peut_voter {
        true => "Adulte",
        false => "Mineur",
    };
    println!("Statut: {}", statut);
}
```

## Caractéristiques de char

```
CHAR EN RUST
├── Taille: 4 bytes (contrairement à C qui utilise 1 byte)
├── Unicode: Supporte tous les caractères Unicode
├── Échappement: '\n', '\t', '\\', '\'', '\"'
└── Comparaison: Peut comparer avec ==, <, >, etc.
```

## Mini-exercices Rustlings

### Exercice 1: Opérateurs Logiques

```rust
fn main() {
    let a = true;
    let b = false;
    // TODO: Calculer (a && b) || (!a)
    let resultat = /* votre code ici */;
    println!("Résultat: {}", resultat);
}
```

## Common Pitfalls

- ❌ **Mistake:** Confondre char et String
  ```rust
  let c = "A";  // C'est une &str, pas un char
  ```
  ✅ **Fix:** Utiliser des guillemets simples pour char
  ```rust
  let c = 'A';  // CORRECT: char
  ```

- ❌ **Mistake:** Penser que char est 1 byte
  ```rust
  // En C: char = 1 byte
  // En Rust: char = 4 bytes (Unicode)
  ```

## Official Resources

- [@official Rust Book - Boolean Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-boolean-type)
- [@official Rust Book - Character Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-character-type)

## Security Notes

Les booléens sont sûrs. Pour les caractères :
- Toujours valider l'input utilisateur
- Attention aux caractères de contrôle (injection)
- Utiliser des méthodes de validation (is_alphabetic, etc.)

