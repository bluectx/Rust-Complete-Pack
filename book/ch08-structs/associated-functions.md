# Associated Functions - Fonctions Statiques! 🎯

## Learning Objectives

- Définir des associated functions (sans self) (c'est simple!)
- Utiliser :: pour appeler
- Créer des constructeurs

## Core Explanation

### For Absolute Beginners - C'est Comme une Usine! 🏭

Imaginez une **usine** 🏭 qui fabrique des objets:
- **Associated function** = L'usine (pas besoin d'un objet existant)
- **Méthode** = Action sur un objet existant
- L'usine crée, l'objet agit!

C'est **exactement** comme les associated functions fonctionnent! C'est **super pratique**!

## Schéma Visuel - Associated Functions

```
┌─────────────────────────────────────────┐
│  🏭 ASSOCIATED FUNCTIONS = USINE 🏭   │
├─────────────────────────────────────────┤
│                                         │
│  Rectangle::square(10)                   │
│         │                                │
│         └─> Usine (crée objet)           │
│                                         │
│  rect.area()                            │
│         │                                │
│         └─> Action (sur objet)          │
│                                         │
│  :: = Usine, . = Action! ✅            │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Usine" - Les associated functions sont comme une usine: elles créent des objets sans avoir besoin d'un objet existant (utilisez ::)!

## Code Examples

### Example 1: Associated Functions

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (pas de self)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Appelé avec ::
    let sq = Rectangle::square(10);
    println!("Carré: {}x{}", sq.width, sq.height);
}
```

## Official Resources

- [@official Rust Book - Associated Functions](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#associated-functions)

