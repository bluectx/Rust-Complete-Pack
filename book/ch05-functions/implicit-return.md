# Retour Implicite - C'est Super Facile! 🎯

## Learning Objectives

- Comprendre que la dernière expression est retournée (c'est magique!)
- Distinguer expression et statement
- Utiliser return explicitement quand nécessaire

## Core Explanation

### For Absolute Beginners - C'est Comme un Cadeau Automatique! 🎁

Imaginez que vous faites quelque chose et que vous **recevez automatiquement** un cadeau 🎁:
- **Retour implicite** = Le dernier résultat est automatiquement retourné
- Pas besoin de dire "return"!
- C'est **super pratique** et **super simple**!

C'est **exactement** comme le retour implicite fonctionne! C'est **super magique**!

## Schéma Visuel - Retour Implicite

```
┌─────────────────────────────────────────┐
│  🎁 RETOUR IMPLICITE = CADEAU 🎁      │
├─────────────────────────────────────────┤
│                                         │
│  fn cinq() -> i32 {                     │
│      5  ← Pas de ; = Expression        │
│  }                                      │
│         │                                │
│         ▼ Automatique                    │
│  Retourne 5!                            │
│                                         │
│  Cadeau automatique! ✨                 │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Cadeau" - Le retour implicite est comme un cadeau automatique: la dernière expression (sans ;) est automatiquement retournée!

## Code Examples

### Example 1: Retour Implicite

```rust
fn cinq() -> i32 {
    5  // Pas de ; = expression = retourné
}

fn main() {
    let x = cinq();
    println!("{}", x);  // 5
}
```

### Example 2: Expression vs Statement

```rust
fn additionner(a: i32, b: i32) -> i32 {
    a + b  // Expression (pas de ;) = retourné
}

fn additionner_explicite(a: i32, b: i32) -> i32 {
    return a + b;  // Statement (avec ;) = retour explicite
}

fn main() {
    println!("{}", additionner(5, 3));
    println!("{}", additionner_explicite(5, 3));
}
```

### Example 3: Erreur Courante

```rust
// ERREUR: ; transforme l'expression en statement
// fn mauvais() -> i32 {
//     5;  // Retourne () au lieu de i32
// }

// CORRECT
fn bon() -> i32 {
    5  // Pas de ;
}
```

## Official Resources

- [@official Rust Book - Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

