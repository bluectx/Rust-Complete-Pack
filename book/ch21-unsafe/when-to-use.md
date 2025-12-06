# Quand Utiliser Unsafe - Attention! ⚠️

## Learning Objectives

- Comprendre quand unsafe est nécessaire (c'est rare!)
- Minimiser l'usage d'unsafe
- Isoler le code unsafe
- Voir les cas d'usage légitimes

## Core Explanation

### For Absolute Beginners - C'est Comme un Hacker Expert! 🕵️

Imaginez un **hacker expert** 🕵️:
- **unsafe** = Mode hacker (seulement pour experts!)
- Utilisez-le SEULEMENT quand vraiment nécessaire
- Sinon → Restez en mode safe!

C'est **exactement** comme unsafe fonctionne! C'est **super dangereux**!

## Schéma Visuel - Quand Utiliser Unsafe

```
┌─────────────────────────────────────────┐
│  ⚠️ QUAND UTILISER UNSAFE ⚠️          │
├─────────────────────────────────────────┤
│                                         │
│  ✅ Légitime:                           │
│  - FFI (appels C)                       │
│  - Optimisations critiques              │
│  - Implémenter abstractions sûres       │
│                                         │
│  ❌ Jamais:                             │
│  - Éviter borrow checker                │
│  - "Optimisation" prématurée            │
│  - Parce que c'est "facile"            │
│                                         │
│  Toujours minimiser! ✅                │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Hacker" - unsafe est comme un mode hacker: utilisez-le seulement quand vraiment nécessaire, sinon restez en mode safe!

## Cas d'Usage Légitimes

### Example 1: FFI

```rust
extern "C" {
    fn c_function();
}

fn safe_wrapper() {
    unsafe {
        c_function();
    }
}
```

### Example 2: Optimisations

```rust
// Accès direct à la mémoire pour performance
unsafe {
    let ptr = data.as_ptr();
    // Manipulation directe
}
```

### Example 3: Implémenter des Abstractions Sûres

```rust
// Utiliser unsafe pour implémenter une API safe
pub fn safe_function(input: &str) -> String {
    unsafe {
        // Code unsafe interne
        // Mais API publique est safe
    }
}
```

## Règles

- **Minimiser** : Utiliser unsafe seulement si nécessaire
- **Isoler** : Contenir dans des fonctions/modules
- **Documenter** : Expliquer pourquoi unsafe est nécessaire
- **Tester** : Tester exhaustivement avec MIRI
- **Wrapper** : Créer des APIs safe par-dessus

## Quand NE PAS Utiliser

- ❌ Juste pour éviter le borrow checker
- ❌ Pour "optimiser" prématurément
- ❌ Parce que c'est "plus facile"

## Official Resources

- [@official Rust Book - Unsafe](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)

## Security Notes

Unsafe contourne les garanties de sécurité :
- Utiliser avec extrême précaution
- Toujours auditer le code unsafe
- Tester avec MIRI
- Documenter tous les invariants
