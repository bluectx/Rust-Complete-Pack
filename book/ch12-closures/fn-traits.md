# Fn, FnMut, FnOnce - Trois Types! 🎭

## Learning Objectives

- Comprendre les trois traits de closure (c'est important!)
- Savoir lequel utiliser
- Comprendre les captures

## Core Explanation

### For Absolute Beginners - C'est Comme Trois Types de Permissions! 🎭

Imaginez **trois types de permissions** 🎭:
- **Fn** = Lire seulement (peut être appelé plusieurs fois)
- **FnMut** = Lire et modifier (peut être appelé plusieurs fois)
- **FnOnce** = Prendre possession (peut être appelé une seule fois)
- C'est **super important** pour comprendre les closures!

C'est **exactement** comme les traits de closure fonctionnent! C'est **super logique**!

## Schéma Visuel - Fn Traits

```
┌─────────────────────────────────────────┐
│  🎭 FN TRAITS = PERMISSIONS 🎭        │
├─────────────────────────────────────────┤
│                                         │
│  Fn:                                    │
│  ┌─────┐                                │
│  │ Lire│ → Plusieurs fois ✅           │
│  └─────┘                                │
│                                         │
│  FnMut:                                 │
│  ┌──────────┐                           │
│  │ Lire+Mod │ → Plusieurs fois ✅      │
│  └──────────┘                           │
│                                         │
│  FnOnce:                                │
│  ┌─────┐                                │
│  │Prend│ → Une seule fois ✅           │
│  └─────┘                                │
│                                         │
│  Permissions différentes! ✅           │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Permissions" - Fn, FnMut, FnOnce sont comme trois types de permissions: lire, lire+modifier, ou prendre possession!

## Code Examples

```rust
// Fn: Peut être appelé plusieurs fois, capture immuablement
let x = 5;
let f = || x + 1;  // Fn
f(); f();  // OK

// FnMut: Capture mutablement
let mut x = 5;
let mut f = || { x += 1; x };  // FnMut
f(); f();  // OK

// FnOnce: Prend ownership
let x = String::from("hello");
let f = move || x;  // FnOnce
f();  // OK
// f();  // ERREUR: x a été déplacé
```

## Official Resources

- [@official Rust Book - Closure Traits](https://doc.rust-lang.org/book/ch13-01-closures.html#closure-traits)

