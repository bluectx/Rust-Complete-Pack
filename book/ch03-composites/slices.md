# Slices - Vues sur Collections! 🎯

## Learning Objectives

- Comprendre ce qu'est une slice (c'est simple!)
- Utiliser &[T] pour les slices
- Comprendre la relation entre slices et tableaux/Vec
- Utiliser les ranges pour créer des slices

## Core Explanation

### For Absolute Beginners - C'est Comme une Fenêtre! 🪟

Imaginez une **fenêtre** 🪟 sur un tableau:
- **Slice** = Une fenêtre qui montre une partie du tableau
- Vous regardez sans posséder (référence)
- C'est **super pratique** pour partager des vues!

C'est **exactement** comme les slices fonctionnent! C'est **super pratique**!

## Schéma Visuel - Slices

```
┌─────────────────────────────────────────┐
│  🪟 SLICE = FENÊTRE 🪟                 │
├─────────────────────────────────────────┤
│                                         │
│  Array: [1, 2, 3, 4, 5]                │
│         ┌───────────┐                  │
│         │ Slice[1..4]│ → [2, 3, 4]     │
│         └───────────┘                  │
│                                         │
│  Vue sans posséder! ✅                 │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Fenêtre" - Une slice est comme une fenêtre: vous regardez une partie d'une collection sans la posséder!

## Key Vocabulary

| Term | Definition |
|------|-----------|
| Slice | Vue sur une partie d'une collection |
| &[T] | Type slice (référence vers une séquence) |
| Range | Plage d'indices (0..5, 0..=5) |

## Code Examples

### Example 1: Slices de Tableaux

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    
    // Créer une slice avec un range
    let slice = &arr[1..4];  // [2, 3, 4]
    
    println!("Slice: {:?}", slice);
    
    // Slices inclusives
    let slice2 = &arr[1..=3];  // [2, 3, 4] (inclusif)
    println!("Slice inclusive: {:?}", slice2);
}
```

### Example 2: Slices de Vec

```rust
fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    
    // Slice d'un Vec
    let slice = &vec[1..4];
    println!("Slice du Vec: {:?}", slice);
    
    // Slice complète
    let slice_complete = &vec[..];
    println!("Slice complète: {:?}", slice_complete);
}
```

### Example 3: Fonctions avec Slices

```rust
fn afficher_slice(slice: &[i32]) {
    for element in slice {
        print!("{} ", element);
    }
    println!();
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let vec = vec![6, 7, 8, 9, 10];
    
    // Même fonction pour array et Vec!
    afficher_slice(&arr[1..4]);
    afficher_slice(&vec[1..4]);
}
```

## Official Resources

- [@official Rust Book - Slices](https://doc.rust-lang.org/book/ch04-03-slices.html)

