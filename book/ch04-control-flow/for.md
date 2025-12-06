# Boucle for - Itérer Facilement! 🔁

## Learning Objectives

- Utiliser for pour itérer sur des collections (c'est simple!)
- Comprendre les ranges (.., ..=)
- Itérer avec index et valeur
- Utiliser for avec iterators

## Core Explanation

### For Absolute Beginners - C'est Comme Parcourir une Liste! 🔁

Imaginez que vous **parcourez** 🔁 une liste:
- **for** = Parcourir chaque élément d'une collection
- Vous visitez chaque élément un par un
- C'est **super pratique** et **super simple**!

C'est **exactement** comme for fonctionne! C'est **super intuitif**!

## Schéma Visuel - For

```
┌─────────────────────────────────────────┐
│  🔁 FOR = PARCOURIR LISTE 🔁           │
├─────────────────────────────────────────┤
│                                         │
│  Collection: [1, 2, 3, 4, 5]           │
│                                         │
│  for élément in collection {           │
│    ┌─────────────┐                      │
│    │ Élément 1   │ → Traiter           │
│    ├─────────────┤                      │
│    │ Élément 2   │ → Traiter           │
│    ├─────────────┤                      │
│    │ Élément 3   │ → Traiter           │
│    └─────────────┘                      │
│                                         │
│  Parcourt tous les éléments! ✅        │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Parcourir Liste" - for parcourt chaque élément d'une collection, comme parcourir une liste d'épicerie!

## Code Examples

### Example 1: For sur Range

```rust
fn main() {
    // Range exclusif (0 à 4)
    for i in 0..5 {
        println!("{}", i);  // 0, 1, 2, 3, 4
    }
    
    // Range inclusif (0 à 5)
    for i in 0..=5 {
        println!("{}", i);  // 0, 1, 2, 3, 4, 5
    }
}
```

### Example 2: For sur Collections

```rust
fn main() {
    let vec = vec![10, 20, 30, 40, 50];
    
    // Itération directe (consomme le Vec)
    for valeur in vec {
        println!("{}", valeur);
    }
    
    // Itération avec référence (ne consomme pas)
    let vec2 = vec![1, 2, 3];
    for valeur in &vec2 {
        println!("{}", valeur);
    }
    // vec2 est toujours valide
    
    // Itération avec index
    for (index, valeur) in vec2.iter().enumerate() {
        println!("Index {}: {}", index, valeur);
    }
}
```

### Example 3: For avec String

```rust
fn main() {
    let s = String::from("hello");
    
    // Itérer sur les caractères
    for c in s.chars() {
        println!("{}", c);
    }
    
    // Itérer sur les bytes
    for b in s.bytes() {
        println!("{}", b);
    }
}
```

## Official Resources

- [@official Rust Book - for](https://doc.rust-lang.org/book/ch03-05-control-flow.html#looping-through-a-collection-with-for)

