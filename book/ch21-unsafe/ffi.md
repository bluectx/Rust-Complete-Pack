# FFI (Foreign Function Interface) - Attention! ⚠️

## Learning Objectives

- Appeler du code C depuis Rust (c'est dangereux!)
- Utiliser extern "C"
- Gérer les appels FFI de manière sûre
- Créer des bindings

## Core Explanation

### For Absolute Beginners - C'est Comme un Traducteur Dangereux! 🌍

Imaginez un **traducteur** 🌍 qui traduit mal:
- **FFI** = Traduire entre Rust et C
- Si mal fait → **vulnérabilités** (buffer overflow, use-after-free)
- Vous devez être **très prudent**!

C'est **exactement** comme FFI fonctionne! C'est **super puissant** mais **super dangereux**!

## Schéma Visuel - FFI

```
┌─────────────────────────────────────────┐
│  🌍 FFI = TRADUCTEUR DANGEREUX 🌍      │
├─────────────────────────────────────────┤
│                                         │
│  Rust (sûr)                             │
│         │                                │
│         ▼ FFI (traduction)              │
│  C (dangereux)                          │
│         │                                │
│         ▼ Si mal fait                   │
│  ⚠️ Vulnérabilités!                     │
│                                         │
│  Toujours valider! ✅                   │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Traducteur" - FFI est comme un traducteur: si mal fait, il peut introduire des vulnérabilités, toujours valider et wrapper!

## Code Examples

### Example 1: Appel C Basique

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Valeur absolue: {}", abs(-3));
    }
}
```

### Example 2: FFI avec Types

```rust
use std::ffi::CString;

extern "C" {
    fn strlen(s: *const i8) -> usize;
}

fn main() {
    let c_string = CString::new("hello").unwrap();
    unsafe {
        let len = strlen(c_string.as_ptr());
        println!("Longueur: {}", len);
    }
}
```

### Example 3: Wrapper Sûr

```rust
extern "C" {
    fn dangerous_c_function(ptr: *mut i32);
}

// Wrapper sûr
fn safe_wrapper(value: &mut i32) {
    unsafe {
        dangerous_c_function(value as *mut i32);
    }
}
```

## Bonnes Pratiques FFI

1. **Wrapper sûr** : Encapsuler dans des fonctions safe
2. **Validation** : Vérifier les inputs
3. **Documentation** : Documenter les invariants
4. **Tests** : Tester exhaustivement

## Official Resources

- [@official Rust Book - FFI](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)

## Security Notes

FFI est très dangereux :
- Pas de vérifications Rust
- Peut causer des crashes
- Peut causer des vulnérabilités
- Toujours wrapper dans du code safe
- Valider tous les inputs
- Tester avec MIRI
