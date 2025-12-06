# FFI - Introduction

## Learning Objectives

- Comprendre FFI (c'est dangereux!)
- Appeler du code C depuis Rust
- Utiliser extern "C"
- Gérer les appels FFI de manière sûre

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

## Official Resources

- [@official Rust Book - FFI](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)

