# Patterns Unsafe - Attention! ⚠️

## Learning Objectives

- Comprendre quand utiliser unsafe (c'est dangereux!)
- Connaître les garanties de sécurité perdues avec unsafe
- Utiliser unsafe de manière sécurisée
- Auditer le code unsafe

## Core Explanation

### For Absolute Beginners - C'est Comme un Hacker! 🕵️

Imaginez un **hacker** 🕵️ qui contourne les sécurités:
- **unsafe** = Mode hacker (plus rapide, mais plus dangereux!)
- Vous perdez les garanties de sécurité
- Vous devez être **très prudent**!

C'est **exactement** comme unsafe fonctionne! C'est **super puissant** mais **super dangereux**!

## Schéma Visuel - Safe vs Unsafe

```
┌─────────────────────────────────────────┐
│  ⚠️ SAFE vs UNSAFE ⚠️                  │
├─────────────────────────────────────────┤
│                                         │
│  Safe Rust:                             │
│  ┌─────────────┐                        │
│  │ Sécurités   │ → Sûr ✅               │
│  │ automatiques│                         │
│  └─────────────┘                        │
│                                         │
│  Unsafe Rust:                           │
│  ┌─────────────┐                        │
│  │ Pas de       │ → Rapide ⚡          │
│  │ sécurités    │   Mais dangereux! ⚠️ │
│  └─────────────┘                        │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Hacker" - unsafe est comme un mode hacker: plus rapide mais plus dangereux, nécessite expertise!

## Key Vocabulary

| Term | Definition |
|------|-----------|
| unsafe | Bloc de code qui contourne certaines vérifications |
| FFI | Foreign Function Interface (appels C) |
| Raw pointer | Pointeur non-géré (*const T, *mut T) |
| Unsafe trait | Trait qui nécessite des garanties manuelles |

## Code Examples

### Example 1: Unsafe Basique

```rust
unsafe fn dangereux() {
    // Code qui nécessite des garanties manuelles
}

fn main() {
    unsafe {
        dangereux();  // Doit être dans un bloc unsafe
    }
}
```

### Example 2: FFI (Foreign Function Interface)

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Valeur absolue de -3 selon C: {}", abs(-3));
    }
}
```

## Bonnes Pratiques

1. **Minimiser l'usage d'unsafe**
2. **Documenter les invariants**
3. **Isoler le code unsafe**
4. **Tester exhaustivement**

## Official Resources

- [@official Rust Book - Unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)

## Security Notes

Unsafe code peut introduire :
- **Memory safety violations**
- **Data races**
- **Undefined behavior**
- Toujours auditer avec MIRI et tests exhaustifs

