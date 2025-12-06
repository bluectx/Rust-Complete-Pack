# Bonnes Pratiques Unsafe - Règles d'Or! ⚠️

## Learning Objectives

- Isoler le code unsafe (c'est essentiel!)
- Documenter les invariants
- Tester exhaustivement
- Utiliser des abstractions sûres

## Core Explanation

### For Absolute Beginners - C'est Comme un Hacker Prudent! 🕵️

Imaginez un **hacker prudent** 🕵️:
- **Isoler** = Mettre le code unsafe dans un endroit séparé
- **Documenter** = Expliquer pourquoi c'est unsafe
- **Tester** = Vérifier avec MIRI
- **Wrapper** = Créer une API safe par-dessus

C'est **exactement** comme utiliser unsafe correctement! C'est **super important**!

## Schéma Visuel - Bonnes Pratiques

```
┌─────────────────────────────────────────┐
│  ⚠️ BONNES PRATIQUES UNSAFE ⚠️         │
├─────────────────────────────────────────┤
│                                         │
│  1. Isoler                              │
│     └─> Code unsafe séparé              │
│                                         │
│  2. Documenter                          │
│     └─> Expliquer pourquoi              │
│                                         │
│  3. Tester                              │
│     └─> Avec MIRI                       │
│                                         │
│  4. Wrapper                             │
│     └─> API safe par-dessus            │
│                                         │
│  Sécurité garantie! ✅                  │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Hacker" - Utilisez unsafe comme un hacker prudent: isolez, documentez, testez, wrapper!

## Règles d'Or

### 1. Minimiser l'Usage

```rust
// ❌ MAUVAIS: Trop d'unsafe
unsafe {
    // Beaucoup de code unsafe
}

// ✅ BON: Unsafe minimal
fn safe_function() {
    let result = unsafe {
        // Minimum nécessaire
    };
    // Reste du code safe
}
```

### 2. Isoler

```rust
// ✅ BON: Unsafe isolé dans un module
mod unsafe_impl {
    pub unsafe fn dangerous() {
        // Code unsafe
    }
}

// API publique safe
pub fn safe_api() {
    unsafe {
        unsafe_impl::dangerous();
    }
}
```

### 3. Documenter

```rust
/// Fonction unsafe qui nécessite:
/// - ptr doit être valide
/// - ptr doit pointer vers au moins size bytes
/// - La mémoire ne doit pas être modifiée pendant l'appel
unsafe fn process_memory(ptr: *const u8, size: usize) {
    // Implémentation
}
```

### 4. Tester avec MIRI

```bash
# Installer MIRI
rustup component add miri

# Tester
cargo miri test
```

### 5. Créer des APIs Sûres

```rust
// Code unsafe interne
unsafe fn unsafe_operation() {
    // ...
}

// API publique safe
pub fn safe_operation(input: &str) -> Result<String, Error> {
    // Validation
    if input.is_empty() {
        return Err(Error::InvalidInput);
    }
    
    // Appel unsafe avec garanties
    unsafe {
        unsafe_operation()
    }
}
```

## Checklist

- [ ] Unsafe est-il vraiment nécessaire?
- [ ] Code unsafe isolé?
- [ ] Invariants documentés?
- [ ] Tests avec MIRI?
- [ ] API publique est safe?
- [ ] Inputs validés?

## Official Resources

- [@official Rust Book - Unsafe Best Practices](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/)

## Security Notes

Unsafe code peut introduire :
- **Memory safety violations**
- **Data races**
- **Undefined behavior**
- Toujours auditer et tester exhaustivement
