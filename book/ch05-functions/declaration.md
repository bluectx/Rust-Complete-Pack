# Déclaration de Fonctions - C'est Super Facile! 🎯

## Learning Objectives

- Déclarer des fonctions avec paramètres typés (c'est simple!)
- Comprendre les valeurs de retour
- Utiliser le retour implicite
- Comprendre la portée des fonctions

## Core Explanation

### For Absolute Beginners - C'est Comme une Recette! 📖

Imaginez une **recette** 📖:
- **Fonction** = Une recette avec des ingrédients (paramètres) et un résultat (retour)
- Vous suivez la recette → Vous obtenez le résultat!
- C'est **super pratique** pour réutiliser du code!

C'est **exactement** comme les fonctions fonctionnent! C'est **super simple**!

## Schéma Visuel - Fonctions

```
┌─────────────────────────────────────────┐
│  📖 FONCTION = RECETTE 📖              │
├─────────────────────────────────────────┤
│                                         │
│  fn additionner(a: i32, b: i32)        │
│         │         │                      │
│         │         └─> Ingrédients       │
│         │                                │
│         └─> Nom de la recette            │
│                                         │
│         → i32                            │
│         └─> Résultat                    │
│                                         │
│  Suivre la recette → Obtenir résultat! ✅│
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Recette" - Une fonction est comme une recette: vous donnez des ingrédients (paramètres) et obtenez un résultat (retour)!

## Code Examples

### Example 1: Fonction Basique

```rust
fn afficher_bonjour() {
    println!("Bonjour!");
}

fn main() {
    afficher_bonjour();
}
```

### Example 2: Paramètres et Retour

```rust
fn additionner(a: i32, b: i32) -> i32 {
    a + b  // Retour implicite (pas de ;)
}

fn main() {
    let resultat = additionner(5, 3);
    println!("5 + 3 = {}", resultat);
}
```

### Example 3: Retour Explicite

```rust
fn soustraire(a: i32, b: i32) -> i32 {
    return a - b;  // Retour explicite (avec ;)
}

fn main() {
    println!("10 - 3 = {}", soustraire(10, 3));
}
```

## Official Resources

- [@official Rust Book - Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

