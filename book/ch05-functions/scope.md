# Portée et Stack Frames - C'est Super Logique! 🎯

## Learning Objectives

- Comprendre la portée des variables (c'est simple!)
- Comprendre les stack frames
- Comprendre la durée de vie des variables
- Voir comment les variables sont libérées

## Core Explanation

### For Absolute Beginners - C'est Comme des Boîtes Empilées! 📦

Imaginez des **boîtes empilées** 📦:
- **Stack frame** = Une boîte (une fonction)
- **Portée** = Ce qui est dans la boîte
- Quand la boîte est fermée, son contenu disparaît!

C'est **exactement** comme les stack frames fonctionnent! C'est **super logique**!

## Schéma Visuel - Stack Frames

```
┌─────────────────────────────────────────┐
│  📦 STACK FRAMES = BOÎTES EMPILÉES 📦  │
├─────────────────────────────────────────┤
│                                         │
│  main()                                 │
│  ┌─────────────┐                        │
│  │ x = 5       │                        │
│  │ fonction1() │                        │
│  │   ┌───────┐ │                        │
│  │   │ y = 10│ │                        │
│  │   └───────┘ │                        │
│  └─────────────┘                        │
│                                         │
│  Chaque boîte a son contenu! ✅        │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Boîtes Empilées" - Les stack frames sont comme des boîtes empilées: chaque fonction a sa propre boîte (frame), et quand elle se termine, la boîte disparaît!

## Code Examples

### Example 1: Portée Locale

```rust
fn main() {
    let x = 5;  // x existe ici
    
    {
        let y = 10;  // y existe seulement dans ce bloc
        println!("x: {}, y: {}", x, y);
    }  // y est libéré ici
    
    // println!("{}", y);  // ERREUR: y n'existe plus
    println!("x: {}", x);  // OK: x existe toujours
}
```

### Example 2: Stack Frames

```rust
fn fonction1() {
    let a = 1;
    println!("Fonction 1, a = {}", a);
    fonction2();
    println!("Retour fonction 1");
}

fn fonction2() {
    let b = 2;
    println!("Fonction 2, b = {}", b);
    fonction3();
    println!("Retour fonction 2");
}

fn fonction3() {
    let c = 3;
    println!("Fonction 3, c = {}", c);
}

fn main() {
    fonction1();
}
```

**Output:**
```
Fonction 1, a = 1
Fonction 2, b = 2
Fonction 3, c = 3
Retour fonction 2
Retour fonction 1
```

## Official Resources

- [@official Rust Book - Scope](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#scope)

