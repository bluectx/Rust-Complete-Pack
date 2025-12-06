# Tuple Structs - C'est Super Facile! 🎯

## Learning Objectives

- Définir des tuple structs (c'est simple!)
- Utiliser les tuple structs
- Comprendre la différence avec les tuples normaux
- Voir les cas d'usage

## Core Explanation

### For Absolute Beginners - C'est Comme Étiqueter! 🏷️

Les tuple structs sont comme des tuples normaux, mais avec un nom de type. C'est utile pour distinguer des tuples qui ont la même structure mais des significations différentes.

**Analogie :**
- **Tuple normal** : `(i32, i32)` - juste deux nombres (sans étiquette)
- **Tuple struct** : `Point(i32, i32)` - deux nombres qui représentent un point (avec étiquette!)

C'est **exactement** comme ça fonctionne! C'est **super pratique**!

## Schéma Visuel - Tuple Structs

```
┌─────────────────────────────────────────┐
│  🏷️ TUPLE STRUCTS = ÉTIQUETTES 🏷️    │
├─────────────────────────────────────────┤
│                                         │
│  Tuple normal:                          │
│  (1000, 1)  → Deux nombres (confus!)   │
│                                         │
│  Tuple struct:                          │
│  Meters(1000)  → Nombre avec sens!      │
│  Kilometers(1) → Nombre avec sens!      │
│                                         │
│  Type-safe! ✅                          │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Étiquettes" - Les tuple structs ajoutent une étiquette (nom de type) aux tuples pour éviter les confusions!

## Code Examples

### Example 1: Tuple Structs Basiques

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("Couleur: ({}, {}, {})", black.0, black.1, black.2);
    println!("Point: ({}, {}, {})", origin.0, origin.1, origin.2);
}
```

### Example 2: Différence avec Tuples

```rust
struct Meters(f64);
struct Kilometers(f64);

fn calculer_distance(m: Meters) -> f64 {
    m.0
}

fn main() {
    let distance_m = Meters(1000.0);
    let distance_km = Kilometers(1.0);
    
    // Impossible de mélanger
    // calculer_distance(distance_km);  // ERREUR: type différent
    
    calculer_distance(distance_m);  // OK
}
```

### Example 3: Tuple Struct avec Un Champ (Newtype)

```rust
struct UserId(u32);
struct ProductId(u32);

fn get_user(id: UserId) {
    println!("User ID: {}", id.0);
}

fn main() {
    let user_id = UserId(42);
    let product_id = ProductId(42);
    
    get_user(user_id);
    // get_user(product_id);  // ERREUR: types différents
}
```

## Cas d'Usage

- **Newtype pattern** : Wrapper un type pour la type safety
- **Unit-like structs** : `struct Marker;` (pas de champs)
- **Distinguer des types similaires** : Éviter les erreurs

## Official Resources

- [@official Rust Book - Tuple Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields)

## Security Notes

Tuple structs améliorent la type safety :
- Empêchent les erreurs de type
- Clarifient l'intention
- Pas de coût en performance

