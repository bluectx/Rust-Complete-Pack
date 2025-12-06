# Vec (Vecteur) - Liste Dynamique! 🎯

## Learning Objectives

- Créer et utiliser Vec (c'est simple!)
- Ajouter et supprimer des éléments
- Itérer sur un Vec
- Comprendre la performance

## Core Explanation

### For Absolute Beginners - C'est Comme une Étagère Élastique! 📚

Imaginez une **étagère élastique** 📚:
- **Vec** = Une étagère qui grandit automatiquement
- Vous ajoutez des livres → L'étagère s'agrandit!
- C'est **super pratique** pour stocker des choses!

C'est **exactement** comme Vec fonctionne! C'est **super pratique**!

## Schéma Visuel - Vec

```
┌─────────────────────────────────────────┐
│  📚 VEC = ÉTAGÈRE ÉLASTIQUE 📚         │
├─────────────────────────────────────────┤
│                                         │
│  Vec vide: []                           │
│                                         │
│  push(1): [1]                           │
│  push(2): [1, 2]                        │
│  push(3): [1, 2, 3]                     │
│                                         │
│  Grandit automatiquement! ✅            │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Étagère Élastique" - Vec est comme une étagère élastique: elle grandit automatiquement quand vous ajoutez des éléments!

## Code Examples

### Example 1: Création et Manipulation

```rust
fn main() {
    // Créer un Vec
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];  // Macro vec!
    
    // Ajouter des éléments
    v.push(1);
    v.push(2);
    v.push(3);
    
    // Accéder aux éléments
    let premier = &v[0];  // Panique si out of bounds
    let deuxieme = v.get(1);  // Retourne Option
    
    // Itérer
    for i in &v {
        println!("{}", i);
    }
}
```

## Official Resources

- [@official Rust Book - Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)

