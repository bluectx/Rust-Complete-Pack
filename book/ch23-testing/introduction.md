# Testing en Rust

## Learning Objectives

- Écrire des tests unitaires
- Écrire des tests d'intégration
- Utiliser les macros assert!
- Organiser les tests
- Utiliser les tests de documentation

## Key Vocabulary

| Term | Definition |
|------|-----------|
| Test unitaire | Test d'une fonction/module isolé |
| Test d'intégration | Test de plusieurs composants ensemble |
| #[test] | Attribut pour marquer une fonction comme test |
| #[should_panic] | Test qui doit paniquer |
| assert! | Macro pour vérifier une condition |

## Core Explanation

### For Absolute Beginners - C'est Comme Goûter Avant de Servir! 🍽️

Les tests, c'est comme **goûter avant de servir** 🍽️:
- Vous testez chaque étape pour vous assurer que tout fonctionne
- Vous vérifiez que les modifications ne cassent rien
- C'est **super important** pour la qualité!

Rust a un framework de test intégré qui permet de :
- Tester automatiquement votre code
- Vérifier que les modifications ne cassent rien
- Documenter le comportement attendu

## Schéma Visuel - Testing

```
┌─────────────────────────────────────────┐
│  🍽️ TESTING = GOÛTER AVANT SERVIR 🍽️ │
├─────────────────────────────────────────┤
│                                         │
│  Code écrit (Plat cuisiné)              │
│         │                                │
│         ▼ Test                           │
│  "Est-ce que ça fonctionne?"            │
│         │                                │
│         ▼ Vérification                   │
│  ✅ Tout est bon!                        │
│                                         │
│  Tests automatiques! ✅                 │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Goûter Avant de Servir" - Les tests sont comme goûter avant de servir: vous vérifiez que tout fonctionne avant de mettre en production!

## Code Examples

### Example 1: Test Unitaire Basique

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
    
    #[test]
    fn test_addition_negatifs() {
        assert_eq!(additionner(-5, 3), -2);
    }
}
```

**Exécuter les tests :**

```bash
cargo test
```

### Example 2: Tests avec Messages

```rust
#[test]
fn test_avec_message() {
    let resultat = additionner(5, 3);
    assert_eq!(resultat, 8, "L'addition de 5 et 3 devrait être 8");
}
```

### Example 3: Test qui Doit Paniquer

```rust
fn diviser(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division par zéro!");
    }
    a / b
}

#[test]
#[should_panic(expected = "Division par zéro")]
fn test_division_par_zero() {
    diviser(10, 0);
}
```

### Example 4: Tests d'Intégration

**Fichier: tests/integration_test.rs**

```rust
use mon_crate;

#[test]
fn test_integration() {
    let resultat = mon_crate::fonction_publique();
    assert_eq!(resultat, 42);
}
```

## Organisation des Tests

```
projet/
├── src/
│   └── lib.rs  (avec #[cfg(test)] mod tests)
└── tests/
    └── integration_test.rs  (tests d'intégration)
```

## Tests de Documentation

```rust
/// Additionne deux nombres
///
/// # Examples
///
/// ```
/// let resultat = additionner(2, 3);
/// assert_eq!(resultat, 5);
/// ```
pub fn additionner(a: i32, b: i32) -> i32 {
    a + b
}
```

**Exécuter les tests de doc :**

```bash
cargo test --doc
```

## Macros Assert

```rust
assert!(condition);           // Vérifie que condition est true
assert_eq!(a, b);            // Vérifie que a == b
assert_ne!(a, b);            // Vérifie que a != b
assert!(condition, "message"); // Avec message personnalisé
```

## Official Resources

- [@official Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)

## Security Notes

Les tests sont essentiels pour la sécurité :
- Tester les cas limites
- Tester les inputs invalides
- Tester les edge cases
- Vérifier la gestion d'erreurs
