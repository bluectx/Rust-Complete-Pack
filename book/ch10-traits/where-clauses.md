# Where Clauses - Organiser les Bounds! 📋

## Learning Objectives

- Utiliser where pour des bounds complexes (c'est pratique!)
- Organiser les bounds de manière lisible

## Core Explanation

### For Absolute Beginners - C'est Comme Organiser un Formulaire! 📋

Imaginez un **formulaire** 📋 bien organisé:
- **where** = Organiser les conditions (bounds) de manière lisible
- Au lieu de tout mettre sur une ligne, vous les organisez
- C'est **super pratique** pour la lisibilité!

C'est **exactement** comme where fonctionne! C'est **super clair**!

## Schéma Visuel - Where Clauses

```
┌─────────────────────────────────────────┐
│  📋 WHERE = ORGANISER 📋              │
├─────────────────────────────────────────┤
│                                         │
│  Sans where (confus):                  │
│  fn f<T: A + B, U: C + D>()            │
│                                         │
│  Avec where (clair):                   │
│  fn f<T, U>()                          │
│  where                                  │
│    T: A + B,                           │
│    U: C + D,                           │
│                                         │
│  Beaucoup plus lisible! ✅             │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Organiser" - where organise les bounds comme un formulaire bien organisé: plus lisible et plus clair!

## Code Examples

```rust
fn fonction_complexe<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // Implémentation
    42
}
```

## Official Resources

- [@official Rust Book - Where Clauses](https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods)

