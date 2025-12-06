# FFI - Parler avec d'Autres Langages! 🌍

## Learning Objectives

- Appeler du code C depuis Rust
- Utiliser extern "C"
- Voir des exemples COOL

## Core Explanation

### For Absolute Beginners - C'est Comme Parler Plusieurs Langues! 🗣️

Imaginez que vous parlez français, mais vous voulez communiquer avec quelqu'un qui parle anglais:
- **FFI** = Un traducteur qui vous aide à parler anglais (C) depuis français (Rust)!
- Vous pouvez utiliser des bibliothèques C depuis Rust!

C'est **exactement** comme FFI fonctionne! C'est **super pratique**!

## Schéma Visuel - FFI

```
┌─────────────────────────────────────────┐
│  🌍 FFI = TRADUCTEUR MULTILINGUE 🌍   │
├─────────────────────────────────────────┤
│                                         │
│  Rust (Français):                       │
│  "Je veux appeler une fonction"         │
│         │                                │
│         ▼ FFI traduit                   │
│         │                                │
│  C (Anglais):                            │
│  "I want to call a function"            │
│                                         │
│  Communication réussie! ✅              │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Traducteur Multilingue" - FFI traduit entre Rust et C, permettant d'utiliser des bibliothèques C depuis Rust!

## Code Examples

### Example 1: Appel C Basique (Super Facile!)

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

