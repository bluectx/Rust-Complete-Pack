# Workspaces - Projets Multiples! 🎯

## Learning Objectives

- Créer des workspaces Cargo (c'est pratique!)
- Organiser plusieurs crates
- Partager des dépendances

## Core Explanation

### For Absolute Beginners - C'est Comme un Immeuble! 🏢

Imaginez un **immeuble** 🏢:
- **Workspace** = L'immeuble (plusieurs appartements/crates)
- Chaque appartement est indépendant mais partage les services
- C'est **super pratique** pour organiser de gros projets!

C'est **exactement** comme les workspaces fonctionnent! C'est **super pratique**!

## Schéma Visuel - Workspaces

```
┌─────────────────────────────────────────┐
│  🏢 WORKSPACE = IMMEUBLE 🏢             │
├─────────────────────────────────────────┤
│                                         │
│  Workspace (Immeuble)                   │
│  ├─> Crate 1 (Appartement 1)           │
│  ├─> Crate 2 (Appartement 2)           │
│  └─> Dépendances partagées (Services)  │
│                                         │
│  Plusieurs crates ensemble! ✅          │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Immeuble" - Un workspace est comme un immeuble: plusieurs crates (appartements) partagent des dépendances (services)!

## Code Examples

```toml
# Cargo.toml à la racine
[workspace]
members = [
    "crate1",
    "crate2",
]

[workspace.dependencies]
tokio = "1.35"
```

## Official Resources

- [@official Rust Book - Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)

