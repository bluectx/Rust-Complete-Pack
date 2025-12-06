# Macros Déclaratives - Code qui Écrit du Code! 🪄

## Learning Objectives

- Comprendre les macros comme des raccourcis magiques
- Créer des macros avec macro_rules!
- Voir des exemples COOL

## Core Explanation

### For Absolute Beginners - C'est Comme un Raccourci Clavier! ⌨️

Imaginez que vous voulez taper "Bonjour" à chaque fois, mais c'est long:
- **Macro** = Un raccourci clavier qui dit "Quand je tape 'bjr', remplace par 'Bonjour'"
- Le compilateur fait la transformation **automatiquement**!

C'est **exactement** comme les macros fonctionnent! C'est **super pratique**!

## Schéma Visuel - Macros

```
┌─────────────────────────────────────────┐
│  🪄 MACROS = RACCOURCIS CLAVIER 🪄      │
├─────────────────────────────────────────┤
│                                         │
│  Vous tapez:                             │
│  say_hello!()                           │
│                                         │
│  Macro transforme en:                    │
│  println!("Hello, World!");             │
│                                         │
│  C'est automatique! ✨                  │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Raccourci Clavier" - Vous tapez un raccourci (macro), le compilateur le remplace automatiquement par le code complet!

## Code Examples

### Example 1: Macro Basique (Super Facile!)

```rust
macro_rules! say_hello {
    () => {
        println!("Hello, World!");
    };
}

fn main() {
    say_hello!();  // Affiche: "Hello, World!"
}
```

## Official Resources

- [@official Rust Book - Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)

