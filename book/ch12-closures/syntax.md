# Syntaxe des Closures - Fonctions Rapides! ⚡

## Learning Objectives

- Comprendre la syntaxe des closures (c'est simple!)
- Utiliser des closures comme fonctions
- Capturer des variables de l'environnement

## Core Explanation

### For Absolute Beginners - C'est Comme une Fonction Rapide! ⚡

Imaginez une **fonction rapide** ⚡ que vous créez sur place:
- **Closure** = Une fonction que vous créez rapidement
- Vous pouvez capturer des variables de l'environnement
- C'est **super pratique** et **super flexible**!

C'est **exactement** comme les closures fonctionnent! C'est **super simple**!

## Schéma Visuel - Syntaxe Closures

```
┌─────────────────────────────────────────┐
│  ⚡ CLOSURE = FONCTION RAPIDE ⚡       │
├─────────────────────────────────────────┤
│                                         │
│  |x| x + 1                              │
│   │  │                                  │
│   │  └─> Corps (code)                  │
│   └─> Paramètres                        │
│                                         │
│  Capture variables de l'environnement! ✅│
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Fonction Rapide" - Une closure est comme une fonction rapide que vous créez sur place, avec capture de variables!

## Code Examples

### Example 1: Closure Basique

```rust
fn main() {
    let add_one = |x| x + 1;
    println!("{}", add_one(5));  // 6
    
    // Avec types explicites
    let add_one_explicit = |x: i32| -> i32 { x + 1 };
    println!("{}", add_one_explicit(5));
}
```

### Example 2: Capture de Variables

```rust
fn main() {
    let x = 10;
    
    // Closure capture x
    let add_x = |y| y + x;
    
    println!("{}", add_x(5));  // 15
}
```

## Official Resources

- [@official Rust Book - Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)

