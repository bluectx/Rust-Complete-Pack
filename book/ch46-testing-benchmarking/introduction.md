# Testing & Benchmarking - Vérifier que Ça Marche! ✅

## Learning Objectives

- Écrire des tests comme un pro
- Utiliser criterion pour benchmarks
- Voir des exemples COOL

## Core Explanation

### For Absolute Beginners - C'est Comme Goûter Avant de Servir! 🍽️

Imaginez que vous cuisinez un **plat**:
- **Tests** = Goûter le plat pour vérifier qu'il est bon
- **Benchmarks** = Mesurer combien de temps ça prend à cuire

C'est **exactement** comme les tests fonctionnent! C'est **super important**!

## Schéma Visuel - Testing & Benchmarking

```
┌─────────────────────────────────────────┐
│  ✅ TESTS = GOÛTER AVANT SERVIR ✅     │
├─────────────────────────────────────────┤
│                                         │
│  Code écrit (Plat cuisiné)              │
│         │                                │
│         ▼ Test                           │
│  "Est-ce que ça fonctionne?"            │
│         │                                │
│         ▼ Benchmark                      │
│  "Combien de temps?"                    │
│                                         │
│  Tout vérifié! ✅                       │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Goûter Avant de Servir" - Vous testez votre code (goûtez) avant de le mettre en production (servir), et vous mesurez les performances (benchmark) pour optimiser!

## Code Examples

### Example 1: Test Basique (Super Facile!)

```rust
fn additionner(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_addition() {
        assert_eq!(additionner(2, 2), 4);
    }
}
```

## Official Resources

- [@official Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)

