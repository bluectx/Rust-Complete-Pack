# Phantom Types - Types Invisibles! 🎯

## Learning Objectives

- Comprendre les phantom types (c'est avancé!)
- Utiliser PhantomData
- Créer des types avec paramètres non-utilisés
- Voir les cas d'usage

## Core Explanation

### For Absolute Beginners - C'est Comme un Fantôme! 👻

Imaginez un **fantôme** 👻:
- **Phantom type** = Un type qui n'existe pas vraiment (comme un fantôme!)
- Il est là pour la sécurité, mais ne prend pas de place
- C'est **super pratique** pour distinguer des types!

C'est **exactement** comme les phantom types fonctionnent! C'est **super magique**!

## Schéma Visuel - Phantom Types

```
┌─────────────────────────────────────────┐
│  👻 PHANTOM TYPES = FANTÔME 👻         │
├─────────────────────────────────────────┤
│                                         │
│  struct Distance<Unit> {                 │
│      value: f64,                        │
│      _unit: PhantomData<Unit>,          │
│  }                                      │
│                                         │
│  Le type Unit existe (pour sécurité)    │
│  mais ne prend pas de place! 👻        │
│                                         │
│  Type safety sans coût! ✅              │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Fantôme" - Les phantom types sont comme des fantômes: ils existent pour la sécurité mais ne prennent pas de place en mémoire!

## Code Examples

### Example 1: PhantomData

```rust
use std::marker::PhantomData;

struct Meter;
struct Kilometer;

struct Distance<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

impl Distance<Meter> {
    fn new_meters(value: f64) -> Self {
        Distance {
            value,
            _unit: PhantomData,
        }
    }
}

impl Distance<Kilometer> {
    fn new_kilometers(value: f64) -> Self {
        Distance {
            value,
            _unit: PhantomData,
        }
    }
}

fn main() {
    let distance_m = Distance::<Meter>::new_meters(1000.0);
    let distance_km = Distance::<Kilometer>::new_kilometers(1.0);
    
    // Impossible de mélanger les types
    // let sum = distance_m.value + distance_km.value;  // Types différents
}
```

## Cas d'Usage

- **Type safety** : Distinguer des types similaires
- **Unités** : Distinguer mètres, kilomètres, etc.
- **États** : Type-state pattern
- **Contraintes** : Ajouter des contraintes au type

## Official Resources

- [PhantomData](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)

