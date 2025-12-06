# Captures de Closures - Attraper des Variables! 🎣

## Learning Objectives

- Comprendre comment les closures capturent (c'est magique!)
- Voir les différents types de captures
- Gérer les captures mutables

## Core Explanation

### For Absolute Beginners - C'est Comme Attraper des Poissons! 🎣

Imaginez que vous **attrapez** 🎣 des poissons (variables):
- **Capture** = Attraper des variables de l'environnement
- Vous pouvez attraper par référence (&), mutable (&mut), ou par move
- C'est **super magique** et **super pratique**!

C'est **exactement** comme les captures fonctionnent! C'est **super flexible**!

## Schéma Visuel - Captures

```
┌─────────────────────────────────────────┐
│  🎣 CAPTURES = ATTRAPER POISSONS 🎣   │
├─────────────────────────────────────────┤
│                                         │
│  Environnement:                         │
│  ┌─────┐                                │
│  │ x=5 │                                │
│  └─────┘                                │
│         │                                │
│         ▼ Closure capture               │
│  || x + 1  → Capture x!                │
│                                         │
│  Types: &, &mut, move ✅               │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Attraper Poissons" - Les closures capturent des variables comme attraper des poissons: vous pouvez les attraper par référence, mutable, ou par move!

## Code Examples

```rust
fn main() {
    let x = 5;
    
    // Capture par référence
    let f1 = || println!("{}", x);
    f1();
    
    // Capture mutable
    let mut y = 10;
    let mut f2 = || { y += 1; };
    f2();
    
    // Capture par move
    let z = String::from("hello");
    let f3 = move || println!("{}", z);
    f3();
}
```

## Official Resources

- [@official Rust Book - Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)

