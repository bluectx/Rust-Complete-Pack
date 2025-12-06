# Gestion Mémoire en Rust

## Learning Objectives

- Comprendre stack vs heap
- Voir comment Rust gère la mémoire
- Optimiser l'utilisation mémoire
- Comprendre les allocations

## Key Vocabulary

| Term | Definition |
|------|-----------|
| Stack | Mémoire rapide pour données de taille fixe |
| Heap | Mémoire flexible pour données de taille variable |
| Allocation | Réservation de mémoire |
| RAII | Resource Acquisition Is Initialization |
| Drop | Trait pour libération automatique |

## Core Explanation

### For Absolute Beginners

La mémoire, c'est comme deux types de stockage :
- **Stack** : Comme un casier - rapide, taille fixe, organisé
- **Heap** : Comme un entrepôt - flexible, taille variable, moins rapide

Rust gère automatiquement la mémoire grâce au système d'ownership, sans garbage collector.

## Stack vs Heap

### Example 1: Stack

```rust
fn main() {
    let x = 5;        // Sur la stack
    let y = x;        // Copie sur la stack
    println!("{}", x); // x toujours valide
}
```

**Caractéristiques Stack :**
- Très rapide
- Taille fixe connue à la compilation
- Libération automatique (LIFO)
- Pas de fragmentation

### Example 2: Heap

```rust
fn main() {
    let s = String::from("hello");  // Sur le heap
    // s possède la mémoire sur le heap
    // Libérée automatiquement quand s sort de portée
}
```

**Caractéristiques Heap :**
- Plus lent (allocation/désallocation)
- Taille variable
- Géré par ownership
- Peut fragmenter

## Diagramme Mémoire

```
STACK                          HEAP
┌─────────┐                   ┌─────────────┐
│ x: 5    │                   │             │
│ y: 5    │                   │  "hello"    │
│ s: ptr──┼───────────────────>│  len: 5     │
│   len:5 │                   │  cap: 5     │
│   cap:5 │                   └─────────────┘
└─────────┘
```

## RAII (Resource Acquisition Is Initialization)

```rust
struct File {
    // Ressource système
}

impl Drop for File {
    fn drop(&mut self) {
        // Libération automatique quand sort de portée
        println!("Fichier fermé");
    }
}

fn main() {
    let file = File::open("test.txt");
    // file sera automatiquement libéré ici
}
```

## Optimisations Mémoire

```rust
// Éviter les allocations inutiles
let vec = Vec::with_capacity(100);  // Pré-allouer

// Utiliser &str au lieu de String quand possible
fn process(s: &str) { }  // Pas d'allocation

// Réutiliser les buffers
let mut buffer = Vec::new();
// Réutiliser buffer au lieu de créer de nouveaux
```

## Official Resources

- [@official Rust Book - Memory](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)

## Security Notes

La gestion mémoire de Rust empêche :
- **Use-after-free** : Ownership garantit la validité
- **Double-free** : Un seul propriétaire
- **Memory leaks** : Libération automatique
- **Buffer overflows** : Vérification des bounds
