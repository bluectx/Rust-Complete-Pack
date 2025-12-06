# Newtype Pattern - Types Sûrs Faciles! 🎯

## Learning Objectives

- Comprendre le pattern newtype (c'est super facile!)
- Créer des types wrapper pour éviter les erreurs
- Voir des exemples COOL

## Core Explanation

### For Absolute Beginners - C'est Comme Étiqueter Vos Boîtes! 📦

Imaginez que vous avez deux boîtes identiques:
- Une contient des **Livres**
- Une contient des **CDs**

Sans étiquettes, vous pourriez les confondre! Le **newtype pattern**, c'est comme mettre une **étiquette** sur chaque boîte pour éviter les erreurs!

C'est **super simple** et **super pratique**!

## Schéma Visuel - Newtype Pattern

```
┌─────────────────────────────────────────┐
│  🎯 NEWTYPE = ÉTIQUETTES 🎯            │
├─────────────────────────────────────────┤
│                                         │
│  Sans Newtype (Dangereux!):            │
│  ┌─────┐  ┌─────┐                      │
│  │ 1000│  │ 1000│  (même valeur!)      │
│  └─────┘  └─────┘                      │
│  ❌ Confusion possible!                 │
│                                         │
│  Avec Newtype (Sûr!):                   │
│  ┌──────────┐  ┌──────────┐            │
│  │Meters(1000)│ │Kilometers(1)│        │
│  └──────────┘  └──────────┘            │
│  ✅ Types différents, pas de confusion! │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Étiquettes de Types" - Le newtype pattern ajoute une étiquette (type wrapper) à vos valeurs pour éviter les confusions, comme étiqueter des boîtes identiques!

## Code Examples

### Example 1: Newtype Basique (Super Facile!)

```rust
// Créer des types wrapper (comme des étiquettes!)
struct Meters(u32);
struct Kilometers(u32);

fn calculer_distance(m: Meters) -> u32 {
    m.0  // Accéder à la valeur avec .0
}

fn main() {
    let distance_m = Meters(1000);
    let distance_km = Kilometers(1);
    
    // ✅ OK: Types compatibles
    calculer_distance(distance_m);
    
    // ❌ ERREUR: Types différents (compilateur protège!)
    // calculer_distance(distance_km);  // ERREUR!
}
```

### Example 2: Newtype avec Types Différents

```rust
// Distinguer différents types de distances
struct Meters(u32);
struct Kilometers(u32);

fn calculer_metres(m: Meters) -> u32 {
    m.0
}

fn main() {
    let distance_m = Meters(1000);
    let distance_km = Kilometers(1);
    
    calculer_metres(distance_m);  // ✅ OK
    
    // ❌ ERREUR: Type différent!
    // calculer_metres(distance_km);  // Le compilateur protège!
}
```

## Avantages

- **Type safety** : Impossible de mélanger les types
- **Clarté** : Code plus lisible
- **Pas de coût** : Aucun overhead en performance
- **Simple** : Facile à utiliser!

## Official Resources

- [Rust Design Patterns - Newtype](https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html)

