# Macros Déclaratives - Introduction

## Learning Objectives

- Comprendre les macros déclaratives (c'est puissant!)
- Créer des macros avec macro_rules!
- Utiliser les patterns de macro
- Voir les exemples courants

## Core Explanation

### For Absolute Beginners - C'est Comme un Raccourci Clavier! ⌨️

Imaginez un **raccourci clavier** ⌨️:
- **Macro déclarative** = Un raccourci qui remplace du code
- Vous tapez le raccourci → Le compilateur le remplace!
- C'est **super pratique** pour éviter la répétition!

C'est **exactement** comme les macros déclaratives fonctionnent! C'est **super puissant**!

## Schéma Visuel - Macros Déclaratives

```
┌─────────────────────────────────────────┐
│  ⌨️ MACROS = RACCOURCIS CLAVIER ⌨️   │
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

**Mnémonique:** "Raccourci" - Les macros déclaratives sont comme des raccourcis clavier: vous tapez un raccourci, le compilateur le remplace par le code complet!

## Code Examples

### Example 1: Macro Basique

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!();
}
```

## Official Resources

- [@official Rust Book - Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)

