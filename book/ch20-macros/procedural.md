# Macros Procédurales - Code Automatique Avancé! 🎯

## Learning Objectives

- Comprendre les macros procédurales (c'est avancé!)
- Créer des derive macros
- Utiliser les attribute macros
- Voir les exemples

## Core Explanation

### For Absolute Beginners - C'est Comme un Assistant Programmé! 🤖

Imaginez un **assistant programmé** 🤖:
- **Macros procédurales** = Un assistant qui écrit du code pour vous
- Vous dites ce que vous voulez → L'assistant génère le code!
- C'est **super puissant** mais **super avancé**!

C'est **exactement** comme les macros procédurales fonctionnent! C'est **super magique**!

## Schéma Visuel - Macros Procédurales

```
┌─────────────────────────────────────────┐
│  🤖 MACROS PROCÉDURALES = ASSISTANT 🤖 │
├─────────────────────────────────────────┤
│                                         │
│  #[derive(Debug, Clone)]                │
│  struct User { ... }                    │
│         │                                │
│         ▼ Assistant génère               │
│  impl Debug for User { ... }            │
│  impl Clone for User { ... }           │
│                                         │
│  Code automatique avancé! ✨            │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Assistant Programmé" - Les macros procédurales sont comme un assistant programmé: vous demandez avec #[derive(...)], et il génère le code automatiquement!

## Code Examples

### Example 1: Utiliser des Derive Macros

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
    };
    
    let json = serde_json::to_string(&user).unwrap();
    println!("{}", json);
}
```

### Example 2: Créer une Derive Macro

**Note:** Les macros procédurales nécessitent un crate séparé.

```rust
// Dans un crate séparé: my_derive
use proc_macro::TokenStream;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Génération de code
    TokenStream::new()
}
```

## Types de Macros Procédurales

1. **Derive macros** : `#[derive(Trait)]`
2. **Attribute macros** : `#[attribute]`
3. **Function-like macros** : `macro!()`

## Official Resources

- [@official Rust Book - Procedural Macros](https://doc.rust-lang.org/book/ch19-06-macros.html#procedural-macros-for-generating-code-from-attributes)

## Security Notes

Les macros procédurales peuvent générer n'importe quel code :
- Vérifier le code généré
- Tester exhaustivement
- Documenter le comportement
- Éviter les injections
