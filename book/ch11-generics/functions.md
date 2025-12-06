# Fonctions Génériques - Code Réutilisable! 🎯

## Learning Objectives

- Définir des fonctions génériques (c'est puissant!)
- Utiliser des type parameters
- Comprendre la monomorphisation

## Core Explanation

### For Absolute Beginners - C'est Comme un Moule Universel! 🍰

Imaginez un **moule universel** 🍰:
- **Fonction générique** = Un moule qui fonctionne avec différents types
- Vous faites un gâteau (i32) ou un cupcake (f64) → Le même moule!
- C'est **super pratique** pour éviter la répétition!

C'est **exactement** comme les fonctions génériques fonctionnent! C'est **super puissant**!

## Schéma Visuel - Génériques

```
┌─────────────────────────────────────────┐
│  🍰 GÉNÉRIQUES = MOULE UNIVERSEL 🍰   │
├─────────────────────────────────────────┤
│                                         │
│  fn largest<T>(list: &[T])              │
│         │                                │
│         ├─> T = i32 → Fonctionne!       │
│         ├─> T = f64 → Fonctionne!       │
│         └─> T = char → Fonctionne!      │
│                                         │
│  Un moule pour tous! ✅                │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Moule Universel" - Les fonctions génériques sont comme un moule universel: elles fonctionnent avec différents types, évitant la répétition!

## Code Examples

### Example 1: Fonction Générique

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("Le plus grand nombre est {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("Le plus grand caractère est {}", result);
}
```

## Official Resources

- [@official Rust Book - Generic Functions](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-function-definitions)

