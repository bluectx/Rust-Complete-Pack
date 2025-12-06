# String vs &str

## Learning Objectives

- Comprendre la différence entre String et &str
- Savoir quand utiliser String vs &str
- Comprendre l'ownership des strings
- Convertir entre String et &str

## Key Vocabulary

| Term | Definition |
|------|-----------|
| String | Type owned (propriétaire) de chaîne, sur le heap |
| &str | Type borrowed (emprunté) de chaîne, string slice |
| String literal | Chaîne littérale ("hello") de type &'static str |

## Core Explanation

### For Absolute Beginners

**String** : Comme posséder un livre - vous pouvez le modifier, le prêter, le détruire
**&str** : Comme emprunter un livre - vous pouvez le lire, mais vous ne le possédez pas

- **String** : Taille variable, sur le heap, mutable
- **&str** : Vue sur une chaîne, immuable, peut pointer vers String ou string literal

## Code Examples

### Example 1: String vs &str

```rust
fn main() {
    // &str: string literal (durée de vie 'static)
    let s1: &str = "hello";
    
    // String: owned string
    let s2: String = String::from("hello");
    
    // Conversion &str -> String
    let s3: String = s1.to_string();
    let s4: String = "world".to_string();
    
    // Conversion String -> &str
    let s5: &str = &s2;
    let s6: &str = s2.as_str();
    
    println!("&str: {}, String: {}", s1, s2);
}
```

### Example 2: Modification

```rust
fn main() {
    // String peut être modifié
    let mut s = String::from("hello");
    s.push_str(", world!");
    s.push('!');
    println!("{}", s);  // "hello, world!!"
    
    // &str ne peut pas être modifié
    let s2 = "hello";
    // s2.push_str(", world!");  // ERREUR: &str est immuable
}
```

### Example 3: Fonctions avec String/&str

```rust
// Préférer &str pour les paramètres (plus flexible)
fn afficher(s: &str) {
    println!("{}", s);
}

fn main() {
    let string_literal = "hello";      // &str
    let owned_string = String::from("world");  // String
    
    // Les deux fonctionnent!
    afficher(string_literal);
    afficher(&owned_string);  // Conversion automatique String -> &str
}
```

## Comparaison

```
STRING
├── Owned (propriétaire)
├── Taille variable
├── Sur le heap
├── Mutable (avec mut)
└── Allocation dynamique

&STR
├── Borrowed (emprunté)
├── Taille fixe (vue)
├── Stack ou heap (dépend de la source)
├── Toujours immuable
└── Pas d'allocation
```

## Official Resources

- [@official Rust Book - Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)

## Security Notes

- String et &str sont sûrs (pas de buffer overflow)
- Toujours valider l'input utilisateur
- Attention aux caractères spéciaux dans les strings

