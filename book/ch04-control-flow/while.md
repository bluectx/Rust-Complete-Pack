# Boucle while - Répéter Tant Que! 🔄

## Learning Objectives

- Utiliser while pour itérer avec condition (c'est simple!)
- Comprendre la différence entre while et loop
- Utiliser while let pour pattern matching

## Core Explanation

### For Absolute Beginners - C'est Comme Répéter Tant Que! 🔄

Imaginez que vous **répétez** 🔄 une action **tant que** une condition est vraie:
- **while** = Répéter tant que la condition est vraie
- Dès que la condition devient fausse, vous arrêtez!
- C'est **super pratique** pour les boucles conditionnelles!

C'est **exactement** comme while fonctionne! C'est **super logique**!

## Schéma Visuel - While

```
┌─────────────────────────────────────────┐
│  🔄 WHILE = RÉPÉTER TANT QUE 🔄       │
├─────────────────────────────────────────┤
│                                         │
│  while condition {                      │
│    ┌─────────────┐                      │
│    │ Condition   │ → Vraie?            │
│    └─────────────┘                      │
│         │ Oui                            │
│         ▼                                │
│    Exécuter code                        │
│         │                                │
│         └─> Retour à condition          │
│                                         │
│  Si condition fausse → Arrêter! ✅     │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Répéter Tant Que" - while répète une action tant que la condition est vraie, comme répéter un exercice tant que vous n'êtes pas fatigué!

## Code Examples

### Example 1: While Basique

```rust
fn main() {
    let mut nombre = 3;
    
    while nombre != 0 {
        println!("{}!", nombre);
        nombre -= 1;
    }
    
    println!("Décollage!");
}
```

### Example 2: While avec Vec

```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    
    while let Some(valeur) = vec.pop() {
        println!("Valeur: {}", valeur);
    }
}
```

### Example 3: Comparaison avec loop

```rust
fn main() {
    let mut compteur = 0;
    
    // Avec while
    while compteur < 5 {
        println!("While: {}", compteur);
        compteur += 1;
    }
    
    // Équivalent avec loop
    let mut compteur2 = 0;
    loop {
        if compteur2 >= 5 {
            break;
        }
        println!("Loop: {}", compteur2);
        compteur2 += 1;
    }
}
```

## Official Resources

- [@official Rust Book - while](https://doc.rust-lang.org/book/ch03-05-control-flow.html#conditional-loops-with-while)

