# Constantes vs Variables

## Learning Objectives

- Comprendre la différence entre const et let
- Savoir quand utiliser const vs let
- Comprendre static variables
- Connaître les règles de nommage

## Key Vocabulary

| Term | Definition |
|------|-----------|
| const | Constante (toujours immuable, valeur connue à la compilation) |
| static | Variable statique (durée de vie 'static) |
| Compile-time | Pendant la compilation |
| Runtime | Pendant l'exécution |

## Core Explanation

### For Absolute Beginners - C'est Comme des Panneaux de Signalisation! 🚦

**Constantes** : Comme des **panneaux de signalisation** 🚦 - fixées pour toujours, connues à l'avance
**Variables** : Comme des boîtes - peuvent changer (si mutables), calculées pendant l'exécution

La différence principale :
- **const** : Valeur connue à la compilation, jamais modifiable
- **let** : Valeur calculée à l'exécution, peut être mutable avec `mut`

## Schéma Visuel - Constantes vs Variables

```
┌─────────────────────────────────────────┐
│  🚦 CONST vs LET = PANNEAUX vs BOÎTES 🚦 │
├─────────────────────────────────────────┤
│                                         │
│  const MAX: u32 = 100;                  │
│  ┌─────────┐                            │
│  │ Panneau │ → Fixé pour toujours!      │
│  └─────────┘                            │
│                                         │
│  let x = 5;                             │
│  ┌─────┐                                │
│  │  5  │ → Peut changer (si mut)        │
│  └─────┘                                │
│                                         │
│  Panneaux vs boîtes! ✅                 │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Panneaux" - Les constantes sont comme des panneaux de signalisation: fixées pour toujours, connues à l'avance!

## Code Examples

### Example 1: Constantes

```rust
// Constantes: toujours en SCREAMING_SNAKE_CASE
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159;

fn main() {
    println!("Points maximum: {}", MAX_POINTS);
    println!("Pi: {}", PI);
    
    // const est TOUJOURS immuable (pas besoin de mut)
    // MAX_POINTS = 200_000;  // ERREUR: const ne peut pas être modifié
}
```

### Example 2: Variables vs Constantes

```rust
fn main() {
    // Variable: peut être calculée à l'exécution
    let x = 5;
    let y = x + 10;  // Calculé à l'exécution
    
    // Constante: doit être connue à la compilation
    const Z: i32 = 5;
    // const W: i32 = x + 10;  // ERREUR: x n'est pas connu à la compilation
    
    println!("Variable y: {}, Constante Z: {}", y, Z);
}
```

### Example 3: Static Variables

```rust
// static: variable globale avec durée de vie 'static
static LANGUAGE: &str = "Rust";
static mut COUNTER: u32 = 0;  // mutable static nécessite unsafe

fn main() {
    println!("Langage: {}", LANGUAGE);
    
    // Accès à static mutable nécessite unsafe
    unsafe {
        COUNTER += 1;
        println!("Compteur: {}", COUNTER);
    }
}
```

## Comparaison

```
CONST vs LET
├── const:
│   ├── Toujours immuable
│   ├── Valeur connue à la compilation
│   ├── Peut être dans n'importe quel scope
│   └── Nommage: SCREAMING_SNAKE_CASE
│
└── let:
    ├── Peut être mutable (avec mut)
    ├── Valeur calculée à l'exécution
    ├── Scope local
    └── Nommage: snake_case
```

## Mini-exercices Rustlings

### Exercice 1: Créer des Constantes

```rust
// TODO: Créer des constantes pour:
// - Taille maximale d'un buffer: 1024
// - Nom de l'application: "MonApp"
// - Version: 1.0

fn main() {
    // Utiliser les constantes ici
}
```

**Solution:**

```rust
const MAX_BUFFER_SIZE: usize = 1024;
const APP_NAME: &str = "MonApp";
const VERSION: f64 = 1.0;

fn main() {
    println!("{} v{} - Buffer: {}", APP_NAME, VERSION, MAX_BUFFER_SIZE);
}
```

## Common Pitfalls

- ❌ **Mistake:** Essayer de calculer une constante à l'exécution
  ```rust
  let x = 5;
  const Y: i32 = x + 10;  // ERREUR
  ```
  ✅ **Fix:** Utiliser let pour les valeurs calculées
  ```rust
  let x = 5;
  let y = x + 10;  // CORRECT
  ```

- ❌ **Mistake:** Oublier le type dans const
  ```rust
  const MAX = 100;  // ERREUR: type requis
  ```
  ✅ **Fix:** Toujours spécifier le type
  ```rust
  const MAX: u32 = 100;  // CORRECT
  ```

## Official Resources

- [@official Rust Book - Constants](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#constants)

## Security Notes

- Les constantes sont sûres (immuables)
- Les static mutables nécessitent unsafe (risque de data races)
- Préférer const à static quand possible

