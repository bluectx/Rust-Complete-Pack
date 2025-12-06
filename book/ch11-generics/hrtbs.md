# HRTBs (Higher-Ranked Trait Bounds) - Lifetimes Avancés! 🎯

## Learning Objectives

- Comprendre les HRTBs (c'est très avancé!)
- Utiliser for<'a> syntax
- Résoudre les problèmes de lifetime complexes

## Core Explanation

### For Absolute Beginners - C'est Comme un Passeport Universel! 🛂

Imaginez un **passeport universel** 🛂:
- **HRTB** = Un passeport qui fonctionne pour TOUS les lifetimes
- Vous dites "pour n'importe quel lifetime" → Ça fonctionne!
- C'est **super puissant** pour les cas complexes!

C'est **exactement** comme les HRTBs fonctionnent! C'est **super avancé**!

## Schéma Visuel - HRTBs

```
┌─────────────────────────────────────────┐
│  🛂 HRTBS = PASSEPORT UNIVERSEL 🛂     │
├─────────────────────────────────────────┤
│                                         │
│  for<'a> Fn(&'a i32)                    │
│         │                                │
│         └─> Fonctionne pour TOUS        │
│             les lifetimes 'a!           │
│                                         │
│  Passeport universel! ✅               │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Passeport Universel" - Les HRTBs sont comme un passeport universel: ils fonctionnent pour tous les lifetimes, résolvant les cas complexes!

## Code Examples

```rust
fn call_with_ref<F>(f: F)
where
    F: for<'a> Fn(&'a i32),
{
    let value = 0;
    f(&value);
}
```

## Official Resources

- [@official Rust Book - Advanced Traits](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html)

