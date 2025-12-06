# Vulnérabilités FFI - Attention! ⚠️

## Learning Objectives

- Comprendre les risques FFI (c'est dangereux!)
- Éviter les vulnérabilités courantes
- Sécuriser les appels FFI
- Tester le code FFI

## Core Explanation

### For Absolute Beginners - C'est Comme un Traducteur Dangereux! 🌍

Imaginez un **traducteur** 🌍 qui traduit mal:
- **FFI** = Traduire entre Rust et C
- Si mal fait → **vulnérabilités** (buffer overflow, use-after-free)
- Vous devez être **très prudent**!

C'est **exactement** comme FFI fonctionne! C'est **super puissant** mais **super dangereux**!

## Schéma Visuel - FFI Vulnérabilités

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

## Vulnérabilités Courantes

### Example 1: Buffer Overflow

```rust
// ❌ DANGEREUX
extern "C" {
    fn strcpy(dest: *mut i8, src: *const i8);
}

// ✅ SÛR: Utiliser des fonctions Rust
use std::ffi::CString;
let c_string = CString::new("hello").unwrap();
```

### Example 2: Use-After-Free

```rust
// ❌ DANGEREUX
unsafe {
    let ptr = c_function();
    // Utiliser ptr après que la mémoire soit libérée
}

// ✅ SÛR: Wrapper avec RAII
struct SafeWrapper {
    ptr: *mut c_void,
}

impl Drop for SafeWrapper {
    fn drop(&mut self) {
        unsafe {
            free_function(self.ptr);
        }
    }
}
```

### Example 3: Validation d'Input

```rust
// ✅ SÛR: Valider avant FFI
fn safe_ffi_call(input: &str) -> Result<(), Error> {
    if input.len() > 100 {
        return Err(Error::InputTooLong);
    }
    
    let c_string = CString::new(input)?;
    unsafe {
        c_function(c_string.as_ptr());
    }
    Ok(())
}
```

## Checklist Sécurité FFI

- [ ] Valider tous les inputs
- [ ] Vérifier les pointeurs
- [ ] Gérer la mémoire correctement
- [ ] Utiliser RAII
- [ ] Tester avec MIRI
- [ ] Documenter les invariants

## Official Resources

- [Rust FFI Guide](https://michael-f-bryan.github.io/rust-ffi-guide/)

## Security Notes

FFI est la source principale de vulnérabilités :
- Toujours valider
- Toujours wrapper
- Toujours tester
- Toujours documenter

