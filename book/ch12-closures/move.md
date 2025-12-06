# Mot-clé move - Prendre Possession! 🎯

## Learning Objectives

- Utiliser move dans les closures (c'est important!)
- Comprendre quand move est nécessaire
- Gérer l'ownership dans les closures

## Core Explanation

### For Absolute Beginners - C'est Comme Déménager! 🚚

Imaginez que vous **déménagez** 🚚:
- **move** = Prendre possession de vos affaires (les déplacer)
- Une fois déménagé, vous n'avez plus les affaires à l'ancien endroit
- C'est **super important** pour les threads!

C'est **exactement** comme move fonctionne dans les closures! C'est **super pratique**!

## Schéma Visuel - move

```
┌─────────────────────────────────────────┐
│  🚚 MOVE = DÉMÉNAGER 🚚                │
├─────────────────────────────────────────┤
│                                         │
│  Avant move:                            │
│  ┌─────┐                                │
│  │ v   │ → Closure emprunte            │
│  └─────┘                                │
│                                         │
│  Avec move:                              │
│  ┌─────┐                                │
│  │ v   │ → Closure prend possession!   │
│  └─────┘                                │
│                                         │
│  v n'est plus valide après! ✅          │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Déménager" - move fait déménager les valeurs dans la closure, comme déménager vos affaires: une fois déménagé, vous n'avez plus les affaires à l'ancien endroit!

## Code Examples

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    
    // move force la closure à prendre ownership
    let handle = thread::spawn(move || {
        println!("Vecteur: {:?}", v);
    });
    
    handle.join().unwrap();
    // v n'est plus valide ici
}
```

## Official Resources

- [@official Rust Book - move Closures](https://doc.rust-lang.org/book/ch13-01-closures.html#capturing-references-or-moving-ownership)

