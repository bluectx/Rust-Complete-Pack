# Tableaux (Arrays) - Collections Fixes! 🎯

## Learning Objectives

- Comprendre les tableaux de taille fixe (c'est simple!)
- Accéder aux éléments d'un tableau
- Comprendre la différence entre array et Vec
- Utiliser les slices sur les tableaux

## Core Explanation

### For Absolute Beginners - C'est Comme une Boîte avec Cases! 📦

Imaginez une **boîte** 📦 avec un nombre fixe de cases:
- **Array** = Une boîte avec un nombre fixe de cases (défini à la compilation)
- Chaque case a un numéro (index)
- Vous ne pouvez pas ajouter ou retirer de cases!
- C'est **super rapide** car tout est sur la stack!

C'est **exactement** comme les arrays fonctionnent! C'est **super simple**!

## Schéma Visuel - Arrays

```
┌─────────────────────────────────────────┐
│  📦 ARRAY = BOÎTE FIXE 📦             │
├─────────────────────────────────────────┤
│                                         │
│  [i32; 5] = Boîte avec 5 cases:        │
│  ┌─────┬─────┬─────┬─────┬─────┐      │
│  │  0  │  1  │  2  │  3  │  4  │      │
│  └─────┴─────┴─────┴─────┴─────┴─────┘  │
│                                         │
│  Taille fixe (connue à la compilation) │
│  Sur la stack (rapide!) ⚡              │
│                                         │
│  Pas de changement de taille! ✅        │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Boîte Fixe" - Un array est comme une boîte avec un nombre fixe de cases: vous ne pouvez pas en ajouter ou en retirer, mais c'est super rapide!

## Key Vocabulary

| Term | Definition |
|------|-----------|
| Array | Collection de taille fixe et connue à la compilation |
| Index | Position d'un élément dans le tableau |
| Bounds checking | Vérification que l'index est valide |
| Slice | Vue sur une partie d'un tableau |

## Code Examples

### Example 1: Création de Tableaux

```rust
fn main() {
    // Tableau de 5 entiers, tous initialisés à 0
    let arr: [i32; 5] = [0, 1, 2, 3, 4];
    
    // Syntaxe courte: [valeur; taille]
    let zeros = [0; 5];  // [0, 0, 0, 0, 0]
    
    println!("Tableau: {:?}", arr);
    println!("Zéros: {:?}", zeros);
}
```

### Example 2: Accès aux Éléments

```rust
fn main() {
    let arr = [10, 20, 30, 40, 50];
    
    // Accès par index
    let premier = arr[0];
    let dernier = arr[4];
    
    println!("Premier: {}, Dernier: {}", premier, dernier);
    
    // Bounds checking à l'exécution
    // let invalide = arr[10];  // Panique à l'exécution
    
    // Utiliser get() pour un accès sûr
    match arr.get(10) {
        Some(v) => println!("Valeur: {}", v),
        None => println!("Index invalide"),
    }
}
```

### Example 3: Itération sur Tableaux

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    
    // Itération avec for
    for element in arr.iter() {
        println!("{}", element);
    }
    
    // Itération avec index
    for (index, value) in arr.iter().enumerate() {
        println!("Index {}: {}", index, value);
    }
    
    // Itération directe (consomme le tableau)
    for element in arr {
        println!("{}", element);
    }
}
```

## Array vs Vec

```
ARRAY
├── Taille fixe (connue à la compilation)
├── Sur la stack
├── Plus rapide
└── Syntaxe: [T; N]

VEC
├── Taille variable
├── Sur le heap
├── Plus flexible
└── Syntaxe: Vec<T>
```

## Official Resources

- [@official Rust Book - Arrays](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type)

## Security Notes

Les tableaux en Rust sont sûrs :
- Bounds checking empêche les buffer overflows
- Accès invalide cause une panique (pas de corruption mémoire)
- Utiliser `get()` pour un accès optionnel sûr

