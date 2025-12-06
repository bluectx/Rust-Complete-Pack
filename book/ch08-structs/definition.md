# Définition de Structs - C'est Super Facile! 🎯

## Learning Objectives

- Définir des structures comme un pro
- Créer des instances de structs
- Accéder aux champs facilement
- Comprendre l'ownership dans les structs

## Core Explanation

### For Absolute Beginners - C'est Comme une Boîte Organisée! 📦

Imaginez une **boîte** 📦 avec des compartiments étiquetés:
- Chaque compartiment = un champ de la struct
- Le nom de la boîte = le nom de la struct
- Vous mettez des valeurs dans chaque compartiment!

C'est **exactement** comme les structs fonctionnent! C'est **super organisé**!

## Schéma Visuel - Struct

```
┌─────────────────────────────────────────┐
│  📦 STRUCT = BOÎTE ORGANISÉE 📦        │
├─────────────────────────────────────────┤
│                                         │
│  struct User {                          │
│    ┌─────────────┐                      │
│    │ username    │ → "alice"           │
│    ├─────────────┤                      │
│    │ email       │ → "alice@ex.com"    │
│    ├─────────────┤                      │
│    │ sign_in     │ → 1                 │
│    ├─────────────┤                      │
│    │ active      │ → true              │
│    └─────────────┘                      │
│  }                                       │
│                                         │
│  Tous les champs ensemble! ✅           │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Boîte" - Une struct est comme une boîte avec des compartiments étiquetés, chaque compartiment contient une valeur!

## Code Examples

### Example 1: Struct Basique

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("alice@example.com"),
        username: String::from("alice"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("Utilisateur: {} ({})", user1.username, user1.email);
}
```

### Example 2: Struct Mutable

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("alice@example.com"),
        username: String::from("alice"),
        active: true,
        sign_in_count: 1,
    };
    
    user1.sign_in_count += 1;
    println!("Connexions: {}", user1.sign_in_count);
}
```

## Avantages des Structs

- **Organisation** : Regrouper des données liées
- **Type safety** : Le compilateur vérifie les types
- **Clarté** : Code plus lisible et maintenable
- **Réutilisabilité** : Créer plusieurs instances

## Pièges Courants

1. **Oublier `mut`** : Impossible de modifier sans `mut`
2. **Ownership** : Les champs String sont déplacés
3. **Champs manquants** : Tous les champs doivent être initialisés

## Official Resources

- [@official Rust Book - Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)

