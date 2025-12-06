# Expressions if/else - C'est Super Facile! 🎯

## Learning Objectives

- Utiliser if/else comme expressions (c'est simple!)
- Comprendre que if retourne une valeur
- Utiliser les conditions booléennes
- Gérer les cas multiples

## Core Explanation

### For Absolute Beginners - C'est Comme un Aiguillage! 🚦

Imaginez un **aiguillage** 🚦:
- **if** = Si condition vraie → Aller à gauche
- **else** = Sinon → Aller à droite
- Le résultat est la valeur retournée!

C'est **exactement** comme if/else fonctionne! C'est **super logique**!

## Schéma Visuel - if/else

```
┌─────────────────────────────────────────┐
│  🚦 IF/ELSE = AIGUILLAGE 🚦            │
├─────────────────────────────────────────┤
│                                         │
│  Condition?                              │
│         │                                │
│    Vrai? │ Faux?                        │
│         │                                │
│    ▼     │     ▼                        │
│  "pair"  │  "impair"                    │
│         │                                │
│  Résultat retourné! ✅                  │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Aiguillage" - if/else est comme un aiguillage: si la condition est vraie, vous allez à gauche, sinon à droite, et vous obtenez un résultat!

## Code Examples

### Example 1: if comme Expression

```rust
fn main() {
    let nombre = 6;
    
    let resultat = if nombre % 2 == 0 {
        "pair"
    } else {
        "impair"
    };
    
    println!("Le nombre est {}", resultat);
}
```

### Example 2: Conditions Multiples

```rust
fn main() {
    let score = 85;
    
    let grade = if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else {
        "F"
    };
    
    println!("Grade: {}", grade);
}
```

## Official Resources

- [@official Rust Book - if Expressions](https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions)

