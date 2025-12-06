# Collections - Introduction

## Learning Objectives

- Comprendre les collections (c'est simple!)
- Utiliser Vec, HashMap, HashSet
- Choisir la bonne collection
- Voir les performances

## Core Explanation

### For Absolute Beginners - C'est Comme Organiser Vos Affaires! 📦

Imaginez organiser vos **affaires** 📦:
- **Vec** = Une étagère élastique (grandit automatiquement)
- **HashMap** = Un dictionnaire (clé → valeur)
- **HashSet** = Une collection unique (pas de doublons)

C'est **exactement** comme les collections fonctionnent! C'est **super pratique**!

## Schéma Visuel - Collections

```
┌─────────────────────────────────────────┐
│  📦 COLLECTIONS = ORGANISATION 📦      │
├─────────────────────────────────────────┤
│                                         │
│  Vec      → Étagère élastique           │
│  HashMap  → Dictionnaire                │
│  HashSet  → Collection unique           │
│                                         │
│  Choisissez la bonne! ✅                │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Organisation" - Les collections sont comme organiser vos affaires: choisissez la bonne collection pour vos besoins!

## Code Examples

### Example 1: Vec

```rust
let mut vec = Vec::new();
vec.push(1);
vec.push(2);
```

### Example 2: HashMap

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("key", "value");
```

## Official Resources

- [@official Rust Book - Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)

