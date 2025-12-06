# Const Generics - Types avec Taille! 🎯

## Learning Objectives

- Utiliser const generics (c'est avancé!)
- Créer des types paramétrés par des constantes

## Core Explanation

### For Absolute Beginners - C'est Comme des Boîtes avec Taille Fixe! 📦

Imaginez des **boîtes avec taille fixe** 📦:
- **Const generics** = Un type paramétré par une constante (comme la taille)
- Array<5> et Array<10> sont des types différents!
- C'est **super pratique** pour la type safety!

C'est **exactement** comme les const generics fonctionnent! C'est **super puissant**!

## Schéma Visuel - Const Generics

```
┌─────────────────────────────────────────┐
│  📦 CONST GENERICS = TAILLE FIXE 📦   │
├─────────────────────────────────────────┤
│                                         │
│  struct Array<T, const N: usize> {      │
│      data: [T; N],                      │
│  }                                      │
│                                         │
│  Array<i32, 5>  → Taille 5              │
│  Array<i32, 10> → Taille 10             │
│                                         │
│  Types différents par taille! ✅        │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Taille Fixe" - Les const generics sont comme des boîtes avec taille fixe: la taille fait partie du type, créant des types différents!

## Code Examples

```rust
struct Array<T, const N: usize> {
    data: [T; N],
}

fn main() {
    let arr: Array<i32, 5> = Array {
        data: [1, 2, 3, 4, 5],
    };
}
```

## Official Resources

- [@official Rust Book - Const Generics](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html)

