# Système de Modules - Organisation du Code! 🎯

## Learning Objectives

- Comprendre le système de modules Rust (c'est important!)
- Organiser le code en modules
- Utiliser mod et use

## Core Explanation

### For Absolute Beginners - C'est Comme Organiser un Placard! 🗄️

Imaginez organiser un **placard** 🗄️:
- **Modules** = Des tiroirs pour organiser vos affaires (code)
- Vous mettez les chaussettes dans un tiroir, les t-shirts dans un autre
- C'est **super pratique** pour organiser!

C'est **exactement** comme les modules fonctionnent! C'est **super pratique**!

## Schéma Visuel - Modules

```
┌─────────────────────────────────────────┐
│  🗄️ MODULES = PLACARD 🗄️              │
├─────────────────────────────────────────┤
│                                         │
│  mod front_of_house {                   │
│      mod hosting { ... }                │
│      mod serving { ... }                │
│  }                                      │
│                                         │
│  Code organisé en tiroirs! ✅           │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Placard" - Les modules sont comme un placard: vous organisez votre code en tiroirs (modules) pour tout garder en ordre!

## Code Examples

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
}
```

## Official Resources

- [@official Rust Book - Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

