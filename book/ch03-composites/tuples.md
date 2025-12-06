# Tuples - Collections Hétérogènes! 🎯

## Learning Objectives

- Comprendre les tuples comme collections hétérogènes (c'est simple!)
- Accéder aux éléments d'un tuple
- Déstructurer des tuples
- Utiliser les tuples pour retourner plusieurs valeurs

## Core Explanation

### For Absolute Beginners - C'est Comme un Paquet! 📦

Imaginez un **paquet** 📦 avec plusieurs choses différentes:
- **Tuple** = Un paquet qui contient plusieurs choses de types différents
- Vous pouvez déballer le paquet (déstructuration)
- C'est **super pratique** pour retourner plusieurs valeurs!

C'est **exactement** comme les tuples fonctionnent! C'est **super simple**!

## Schéma Visuel - Tuples

```
┌─────────────────────────────────────────┐
│  📦 TUPLE = PAQUET 📦                  │
├─────────────────────────────────────────┤
│                                         │
│  (i32, f64, bool)                       │
│  ┌─────┐ ┌─────┐ ┌─────┐              │
│  │ 500 │ │ 6.4 │ │true │              │
│  └─────┘ └─────┘ └─────┘              │
│                                         │
│  Types différents ensemble! ✅          │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Paquet" - Un tuple est comme un paquet: plusieurs choses de types différents ensemble, vous pouvez déballer (déstructurer)!

## Key Vocabulary

| Term | Definition |
|------|-----------|
| Tuple | Collection ordonnée de valeurs de types différents |
| Déstructuration | Extraction des valeurs d'un tuple |
| Hétérogène | Contient des types différents |

## Code Examples

### Example 1: Création et Accès

```rust
fn main() {
    let tup: (i32, f64, bool) = (500, 6.4, true);
    
    // Accès par index
    let cinq_cents = tup.0;
    let six_quatre = tup.1;
    let vrai = tup.2;
    
    println!("{}, {}, {}", cinq_cents, six_quatre, vrai);
}
```

### Example 2: Déstructuration

```rust
fn main() {
    let tup = (500, 6.4, true);
    
    // Déstructuration
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);
}
```

### Example 3: Retour Multiple

```rust
fn calculer() -> (i32, i32) {
    (5, 3)  // Retourne un tuple
}

fn main() {
    let (a, b) = calculer();
    println!("a = {}, b = {}", a, b);
}
```

## Official Resources

- [@official Rust Book - Tuples](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type)

