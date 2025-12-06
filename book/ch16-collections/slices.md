# Slices dans Collections - Vues Magiques! 🎯

## Learning Objectives

- Utiliser des slices avec Vec et String (c'est simple!)
- Comprendre &[T] et &str
- Manipuler des slices

## Core Explanation

### For Absolute Beginners - C'est Comme une Fenêtre! 🪟

Imaginez une **fenêtre** 🪟 sur une collection:
- **Slice** = Une fenêtre qui montre une partie de la collection
- Vous regardez sans posséder (référence)
- C'est **super pratique** pour partager des vues!

C'est **exactement** comme les slices fonctionnent! C'est **super pratique**!

## Schéma Visuel - Slices

```
┌─────────────────────────────────────────┐
│  🪟 SLICES = FENÊTRE 🪟                │
├─────────────────────────────────────────┤
│                                         │
│  Vec: [1, 2, 3, 4, 5]                  │
│         ┌───────────┐                  │
│         │ Slice[1..4]│ → [2, 3, 4]     │
│         └───────────┘                  │
│                                         │
│  Vue sans posséder! ✅                 │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Fenêtre" - Une slice est comme une fenêtre: vous regardez une partie d'une collection sans la posséder!

## Code Examples

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let slice = &v[1..4];  // [2, 3, 4]
    
    let s = String::from("hello world");
    let word = &s[0..5];  // "hello"
    
    // Slice de string (UTF-8 safe)
    let chars: Vec<char> = s.chars().collect();
}
```

## Official Resources

- [@official Rust Book - Slices](https://doc.rust-lang.org/book/ch04-03-slices.html)

