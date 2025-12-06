# Raw Pointers - Attention! ⚠️

## Learning Objectives

- Comprendre les raw pointers (c'est dangereux!)
- Utiliser *const T et *mut T
- Convertir entre références et raw pointers
- Voir les cas d'usage

## Core Explanation

### For Absolute Beginners - C'est Comme un Pointeur Laser Sans Sécurité! 🔴

Imaginez un **pointeur laser** 🔴 sans sécurité:
- **Raw pointer** = Un pointeur sans protection
- Vous pouvez pointer n'importe où (dangereux!)
- Utilisez-le SEULEMENT si vraiment nécessaire!

C'est **exactement** comme les raw pointers fonctionnent! C'est **super dangereux**!

## Schéma Visuel - Raw Pointers

```
┌─────────────────────────────────────────┐
│  🔴 RAW POINTERS = POINTEUR LASER 🔴  │
├─────────────────────────────────────────┤
│                                         │
│  *const T  → Pointeur immuable          │
│  *mut T    → Pointeur mutable           │
│                                         │
│  ⚠️ Pas de vérifications!               │
│  ⚠️ Peut pointer n'importe où!         │
│  ⚠️ Nécessite unsafe!                  │
│                                         │
│  Utiliser avec précaution! ⚠️          │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Pointeur Laser" - Les raw pointers sont comme un pointeur laser sans sécurité: puissant mais dangereux, utilisez avec précaution!

## Code Examples

### Example 1: Raw Pointers Basiques

```rust
fn main() {
    let x = 5;
    let raw_ptr = &x as *const i32;
    
    unsafe {
        println!("Valeur: {}", *raw_ptr);
    }
}
```

### Example 2: Raw Pointers Mutables

```rust
fn main() {
    let mut x = 5;
    let raw_ptr = &mut x as *mut i32;
    
    unsafe {
        *raw_ptr = 10;
        println!("Nouvelle valeur: {}", x);
    }
}
```

### Example 3: Conversion

```rust
fn main() {
    let x = 5;
    
    // Référence vers raw pointer
    let raw = &x as *const i32;
    
    // Raw pointer vers référence (unsafe)
    unsafe {
        let ref_back = &*raw;
        println!("{}", ref_back);
    }
}
```

## Types de Raw Pointers

```
*const T  → Raw pointer immuable
*mut T    → Raw pointer mutable
```

## Règles

- Raw pointers peuvent être null
- Raw pointers ne sont pas suivis par le borrow checker
- Accès nécessite unsafe
- Pas de garanties de sécurité

## Official Resources

- [@official Rust Book - Raw Pointers](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#dereferencing-a-raw-pointer)

## Security Notes

Raw pointers sont dangereux :
- Pas de vérification de bounds
- Pas de vérification de lifetime
- Peuvent pointer vers mémoire invalide
- Utiliser seulement quand absolument nécessaire
