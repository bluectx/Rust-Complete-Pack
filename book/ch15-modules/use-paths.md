# use et Paths - Imports Magiques! 🎯

## Learning Objectives

- Utiliser use pour importer (c'est simple!)
- Comprendre les paths absolus et relatifs
- Organiser les imports

## Core Explanation

### For Absolute Beginners - C'est Comme un Raccourci! 🗺️

Imaginez un **raccourci** 🗺️:
- **use** = Créer un raccourci vers quelque chose
- Au lieu de dire "std::collections::HashMap", dites juste "HashMap"!
- C'est **super pratique** pour éviter la répétition!

C'est **exactement** comme use fonctionne! C'est **super pratique**!

## Schéma Visuel - use

```
┌─────────────────────────────────────────┐
│  🗺️ USE = RACCOURCI 🗺️                 │
├─────────────────────────────────────────┤
│                                         │
│  use std::collections::HashMap;          │
│         │                                │
│         └─> Crée un raccourci            │
│                                         │
│  HashMap::new()  ← Plus court!          │
│                                         │
│  Raccourci pratique! ✅                 │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Raccourci" - use crée un raccourci vers quelque chose, évitant de répéter le chemin complet!

## Code Examples

```rust
use std::collections::HashMap;

// Path absolu
use crate::front_of_house::hosting;

// Path relatif
use self::front_of_house::hosting;

// Re-export
pub use crate::front_of_house::hosting;
```

## Official Resources

- [@official Rust Book - use](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html)

