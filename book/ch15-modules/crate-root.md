# Crate Root - Point d'Entrée! 🎯

## Learning Objectives

- Comprendre le crate root (c'est important!)
- Organiser les fichiers
- Utiliser mod.rs ou fichiers séparés

## Core Explanation

### For Absolute Beginners - C'est Comme la Porte d'Entrée! 🚪

Imaginez une **porte d'entrée** 🚪:
- **Crate root** = La porte d'entrée de votre projet
- Tout commence ici (main.rs ou lib.rs)
- C'est **super important** pour organiser!

C'est **exactement** comme le crate root fonctionne! C'est **super logique**!

## Schéma Visuel - Crate Root

```
┌─────────────────────────────────────────┐
│  🚪 CRATE ROOT = PORTE D'ENTRÉE 🚪     │
├─────────────────────────────────────────┤
│                                         │
│  src/main.rs  ← Crate root              │
│         │                                │
│         ├─> mod my_module;              │
│         └─> fn main() { ... }            │
│                                         │
│  Tout commence ici! ✅                  │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Porte d'Entrée" - Le crate root est comme la porte d'entrée: tout votre projet commence ici (main.rs ou lib.rs)!

## Code Examples

```rust
// src/main.rs ou src/lib.rs est le crate root
// On peut déclarer des modules ici

mod my_module;

fn main() {
    my_module::function();
}
```

## Official Resources

- [@official Rust Book - Crate Root](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)

