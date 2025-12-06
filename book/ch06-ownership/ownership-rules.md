# Règles d'Ownership

## Learning Objectives

- Comprendre les trois règles fondamentales de l'ownership
- Distinguer ownership et borrowing
- Comprendre le concept de move
- Appréhender la gestion automatique de la mémoire

## Key Vocabulary

| Term | Definition |
|------|-----------|
| Ownership | Système de propriété unique des valeurs en mémoire |
| Move | Transfert de propriété d'une variable à une autre |
| Copy | Copie de la valeur (pour types Copy) |
| Borrow | Emprunt temporaire d'une valeur sans prendre ownership |
| Stack | Mémoire rapide pour données de taille fixe |
| Heap | Mémoire pour données de taille variable |

## Core Explanation

### For Absolute Beginners - C'est Comme un Système de Bibliothèque! 📚

L'ownership en Rust, c'est comme un **système de bibliothèque** 📚:
- **Une seule personne peut emprunter un livre à la fois** (ownership unique)
- **Vous pouvez lire le livre sans l'emprunter** (borrowing/emprunt)
- **Quand vous rendez le livre, quelqu'un d'autre peut l'emprunter** (move)

Rust garantit la sécurité mémoire sans garbage collector en suivant strictement ces règles.

## Schéma Visuel - Ownership Rules

```
┌─────────────────────────────────────────┐
│  📚 OWNERSHIP = BIBLIOTHÈQUE 📚       │
├─────────────────────────────────────────┤
│                                         │
│  Règle 1: Un livre, un emprunteur        │
│  ┌─────┐                                │
│  │Livre│ → Alice (propriétaire)        │
│  └─────┘                                │
│                                         │
│  Règle 2: Move = Rendre le livre        │
│  Alice → Bob (nouveau propriétaire)     │
│                                         │
│  Règle 3: Libération automatique        │
│  Bob sort → Livre rangé!                │
│                                         │
│  Système sûr! ✅                        │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Bibliothèque" - L'ownership est comme un système de bibliothèque: un livre, un emprunteur, libération automatique!

## Les Trois Règles d'Ownership

1. **Chaque valeur a un et un seul propriétaire**
2. **Il ne peut y avoir qu'un seul propriétaire à la fois**
3. **Quand le propriétaire sort de portée, la valeur est libérée**

## Code Examples

### Example 1: Ownership Basique

```rust
fn main() {
    let s = String::from("hello");  // s est propriétaire de la String
    // s possède la String
    
    // Quand s sort de portée, la mémoire est automatiquement libérée
}  // s n'existe plus, mémoire libérée
```

### Example 2: Move (Transfert d'Ownership)

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 est "déplacé" vers s2, s1 n'est plus valide
    
    // println!("{}", s1);  // ERREUR: s1 n'est plus valide
    println!("{}", s2);  // OK: s2 possède maintenant la String
}
```

**Explanation:**

- `String` n'implémente pas `Copy`, donc c'est un **move**
- Après le move, `s1` n'est plus utilisable
- La mémoire n'est libérée qu'une seule fois (par s2)

### Example 3: Copy vs Move

```rust
fn main() {
    // Types Copy (copie automatique)
    let x = 5;
    let y = x;  // Copie, x est toujours valide
    println!("x = {}, y = {}", x, y);  // OK
    
    // Types Move (transfert d'ownership)
    let s1 = String::from("hello");
    let s2 = s1;  // Move, s1 n'est plus valide
    // println!("{}", s1);  // ERREUR
    println!("{}", s2);  // OK
}
```

## Comparaisons avec C/C++

### En C/C++ (Gestion Manuelle)

```c
// C: Gestion manuelle, propice aux erreurs
char* s = malloc(6);
strcpy(s, "hello");
// Oublier free(s) = fuite mémoire
// Utiliser après free = use-after-free
free(s);
```

### En Rust (Automatique et Sûr)

```rust
// Rust: Gestion automatique, vérifiée à la compilation
let s = String::from("hello");
// Mémoire libérée automatiquement quand s sort de portée
// Impossible d'avoir use-after-free (vérifié à la compilation)
```

## Diagramme de Mémoire

```
STACK                          HEAP
┌─────────┐                   ┌─────────────┐
│ s1      │                   │ "hello"     │
│ ptr ────┼───────────────────>│ len: 5      │
│ len: 5  │                   │ capacity: 5 │
│ cap: 5  │                   └─────────────┘
└─────────┘

Après move vers s2:

STACK                          HEAP
┌─────────┐                   ┌─────────────┐
│ s1      │                   │ "hello"     │
│ (invalide)                  │ len: 5      │
└─────────┘                   │ capacity: 5 │
┌─────────┐                   └─────────────┘
│ s2      │                          ▲
│ ptr ────┼──────────────────────────┘
│ len: 5  │
│ cap: 5  │
└─────────┘
```

## Mini-exercices Rustlings

### Exercice 1: Comprendre le Move

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    // TODO: Que se passe-t-il si on essaie d'utiliser s1 ici?
    println!("{}", s1);  // Décommentez et voyez l'erreur
}
```

## Exercises

### Exercise 1: Ownership Transfer

**Level:** ⭐⭐ Intermediate

**Challenge:** Créer une fonction qui prend ownership d'une String et l'affiche.

### Exercise 2: Copy vs Move

**Level:** ⭐⭐ Intermediate

**Challenge:** Expliquer pourquoi `i32` peut être copié mais `String` ne peut pas.

## Cheatsheet

```
OWNERSHIP RULES
├── 1. Une valeur = un propriétaire
├── 2. Un seul propriétaire à la fois
└── 3. Libération automatique en sortie de portée

MOVE vs COPY
├── Copy → Types simples (i32, bool, char, tuples de Copy)
└── Move → Types complexes (String, Vec, etc.)

TYPES COPY
├── Tous les entiers (i32, u64, etc.)
├── Flottants (f32, f64)
├── Booléens (bool)
├── Caractères (char)
└── Tuples de types Copy
```

## Common Pitfalls

- ❌ **Mistake:** Utiliser une variable après un move
  ```rust
  let s1 = String::from("hello");
  let s2 = s1;
  println!("{}", s1);  // ERREUR
  ```
  ✅ **Fix:** Utiliser la nouvelle variable ou utiliser borrowing
  ```rust
  let s1 = String::from("hello");
  let s2 = s1;
  println!("{}", s2);  // CORRECT
  ```

- ❌ **Mistake:** Penser que tout est copié comme en Python
  ```rust
  let s1 = String::from("hello");
  let s2 = s1;  // Move, pas copie!
  ```
  ✅ **Fix:** Comprendre la différence entre Copy et Move

## Official Resources

- [@official Rust Book - Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [@official Rust Book - Ownership Rules](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules)

## Security Notes

Le système d'ownership empêche :
- **Use-after-free** : Impossible d'utiliser une valeur après move
- **Double-free** : Un seul propriétaire peut libérer
- **Memory leaks** : Libération automatique garantie
- **Data races** : Ownership unique empêche les accès concurrents non sécurisés

