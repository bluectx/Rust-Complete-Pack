# Move vs Copy dans Structs - C'est Super Facile! 🎯

## Learning Objectives

- Comprendre comment les structs sont déplacés ou copiés (c'est simple!)
- Connaître les types Copy dans les structs
- Gérer l'ownership avec les structs
- Voir la différence entre Copy et Clone

## Core Explanation

### For Absolute Beginners - C'est Comme Photocopier ou Donner! 📄

Quand vous assignez une struct à une autre variable, Rust doit décider : est-ce une copie ou un déplacement ? Cela dépend si le type implémente le trait `Copy`.

**Analogie :**
- **Copy** : Comme **photocopier** un document - l'original reste
- **Move** : Comme **donner** un livre - vous ne l'avez plus

C'est **exactement** comme ça fonctionne! C'est **super logique**!

## Schéma Visuel - Move vs Copy

```
┌─────────────────────────────────────────┐
│  📄 MOVE vs COPY 📄                     │
├─────────────────────────────────────────┤
│                                         │
│  COPY (photocopier):                    │
│  Original → Copie                       │
│  ┌─────┐      ┌─────┐                  │
│  │ Doc │  →   │ Doc │  (les deux OK!)  │
│  └─────┘      └─────┘                  │
│                                         │
│  MOVE (donner):                         │
│  Original → Nouveau                     │
│  ┌─────┐      ┌─────┐                  │
│  │ Liv │  →   │ Liv │  (original ❌)  │
│  └─────┘      └─────┘                  │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Photocopier" - Copy = photocopier (les deux existent), Move = donner (l'original n'existe plus)!

## Code Examples

### Example 1: Struct Copy

```rust
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 0, y: 0 };
    let p2 = p1;  // Copy car Point implémente Copy
    println!("p1: ({}, {})", p1.x, p1.y);  // OK: p1 toujours valide
    println!("p2: ({}, {})", p2.x, p2.y);  // OK: p2 est une copie
}
```

### Example 2: Struct Move

```rust
struct Person {
    name: String,  // String n'est pas Copy
}

fn main() {
    let p1 = Person {
        name: String::from("Alice"),
    };
    let p2 = p1;  // Move: p1 est déplacé vers p2
    // println!("{}", p1.name);  // ERREUR: p1 n'est plus valide
    println!("{}", p2.name);  // OK: p2 possède maintenant
}
```

### Example 3: Copy vs Clone

```rust
#[derive(Clone)]
struct Data {
    value: String,
}

fn main() {
    let d1 = Data {
        value: String::from("hello"),
    };
    let d2 = d1.clone();  // Clone explicite (copie coûteuse)
    println!("d1: {}", d1.value);  // OK
    println!("d2: {}", d2.value);  // OK
}
```

## Règles Copy

Un type peut être `Copy` si :
- Tous ses champs sont `Copy`
- Il n'implémente pas `Drop`
- Il est petit et rapide à copier

## Official Resources

- [@official Rust Book - Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)

## Security Notes

Copy est sûr :
- Pas de double-free (copie bitwise)
- Pas de use-after-free (original reste valide)
- Performance garantie (copie rapide)

