# Commentaires de Documentation - C'est Super Important! 🎯

## Learning Objectives

- Écrire des commentaires de documentation (c'est essentiel!)
- Utiliser /// pour documenter les fonctions
- Générer la documentation avec cargo doc
- Comprendre les exemples dans la doc

## Core Explanation

### For Absolute Beginners - C'est Comme un Manuel d'Instruction! 📖

Imaginez un **manuel d'instruction** 📖:
- **Documentation** = Le manuel qui explique comment utiliser votre code
- **///** = Les instructions écrites
- **cargo doc** = Générer le manuel automatiquement!

C'est **exactement** comme la documentation fonctionne! C'est **super important**!

## Schéma Visuel - Documentation

```
┌─────────────────────────────────────────┐
│  📖 DOCUMENTATION = MANUEL 📖         │
├─────────────────────────────────────────┤
│                                         │
│  /// Additionne deux nombres            │
│  fn additionner(a: i32, b: i32)        │
│         │                                │
│         ▼ cargo doc génère               │
│  Manuel HTML avec instructions!         │
│                                         │
│  Documentation automatique! ✨          │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Manuel" - La documentation est comme un manuel d'instruction: vous écrivez des instructions (///), et cargo doc génère le manuel automatiquement!

## Code Examples

### Example 1: Documentation Basique

```rust
/// Additionne deux nombres entiers.
///
/// # Arguments
///
/// * `a` - Le premier nombre
/// * `b` - Le deuxième nombre
///
/// # Returns
///
/// La somme de `a` et `b`
///
/// # Examples
///
/// ```
/// let resultat = additionner(5, 3);
/// assert_eq!(resultat, 8);
/// ```
fn additionner(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", additionner(5, 3));
}
```

### Example 2: Documentation de Module

```rust
//! # Module de Calcul
//!
//! Ce module fournit des fonctions mathématiques de base.

/// Calcule le carré d'un nombre.
pub fn carre(x: i32) -> i32 {
    x * x
}
```

### Example 3: Générer la Documentation

```bash
# Générer la documentation
cargo doc

# Ouvrir dans le navigateur
cargo doc --open
```

## Official Resources

- [@official Rust Book - Documentation](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#documentation-comments)

