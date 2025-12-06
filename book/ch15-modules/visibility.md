# Visibilité - Contrôle d'Accès! 🎯

## Learning Objectives

- Comprendre pub, pub(crate), pub(super) (c'est important!)
- Contrôler la visibilité
- Organiser les APIs publiques

## Core Explanation

### For Absolute Beginners - C'est Comme des Portes! 🚪

Imaginez des **portes** 🚪:
- **pub** = Porte ouverte (tout le monde peut entrer)
- **privé** = Porte fermée (seulement vous pouvez entrer)
- C'est **super important** pour la sécurité!

C'est **exactement** comme la visibilité fonctionne! C'est **super pratique**!

## Schéma Visuel - Visibilité

```
┌─────────────────────────────────────────┐
│  🚪 VISIBILITÉ = PORTES 🚪              │
├─────────────────────────────────────────┤
│                                         │
│  pub fn public() { ... }                │
│  └─> Porte ouverte (tous peuvent entrer)│
│                                         │
│  fn private() { ... }                   │
│  └─> Porte fermée (seulement vous)     │
│                                         │
│  Contrôle d'accès! ✅                   │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Portes" - La visibilité est comme des portes: pub = porte ouverte (public), privé = porte fermée (privé)!

## Code Examples

```rust
mod outer {
    pub fn public() {}
    fn private() {}
    
    pub mod inner {
        pub fn public() {}
        pub(super) fn semi_private() {}
    }
}
```

## Official Resources

- [@official Rust Book - Visibility](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)

