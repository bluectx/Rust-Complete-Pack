# Interior Mutability - C'est Super Puissant! 🎯

## Learning Objectives

- Comprendre le concept d'interior mutability (c'est avancé!)
- Utiliser RefCell et Cell
- Connaître les cas d'usage
- Voir les alternatives

## Core Explanation

### For Absolute Beginners - C'est Comme un Cornichon dans un Bocal! 🥒

Imaginez un **cornichon** 🥒 dans un **bocal**:
- **Bocal** = Immutable (vous ne pouvez pas changer le bocal)
- **Cornichon** = Modifiable (mais vous pouvez modifier le cornichon à l'intérieur!)

C'est **exactement** comme interior mutability fonctionne! C'est **super flexible**!

## Schéma Visuel - Interior Mutability

```
┌─────────────────────────────────────────┐
│  🥒 INTERIOR MUTABILITY = CORNICHON 🥒 │
├─────────────────────────────────────────┤
│                                         │
│  let data = RefCell::new(5);            │
│  ┌─────────────┐                        │
│  │ Bocal       │ (immuable)            │
│  │ ┌─────────┐ │                        │
│  │ │ Cornichon│ │ (modifiable!)         │
│  │ └─────────┘ │                        │
│  └─────────────┘                        │
│                                         │
│  Vérification à l'exécution! ⚠️        │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Cornichon" - Interior mutability est comme un cornichon dans un bocal: le bocal est immuable mais vous pouvez modifier le cornichon à l'intérieur, avec vérification à l'exécution!

## Code Examples

### Example 1: RefCell

```rust
use std::cell::RefCell;

// RefCell: borrows vérifiés à l'exécution
let data = RefCell::new(5);
let borrow = data.borrow_mut();
*borrow += 1;
```

### Example 2: Cell

```rust
use std::cell::Cell;

// Cell: Copy types seulement, pas de borrows
let cell = Cell::new(5);
cell.set(10);
let value = cell.get();  // Pas de référence nécessaire
```

### Example 3: Cas d'Usage

```rust
use std::cell::RefCell;

// Cas d'usage: Mock objects
struct MockLogger {
    logs: RefCell<Vec<String>>,
}

impl MockLogger {
    fn log(&self, msg: &str) {
        // self n'est pas mutable, mais on modifie logs
        self.logs.borrow_mut().push(msg.to_string());
    }
}
```

## Comparaison

```
REFCELL
├── Borrows vérifiés à l'exécution
├── Peut paniquer
├── Tous types
└── borrow() / borrow_mut()

CELL
├── Pas de borrows (get/set)
├── Copy types seulement
├── Pas de panics
└── get() / set()
```

## Official Resources

- [@official Rust Book - Interior Mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)

## Security Notes

Interior mutability contourne les vérifications à la compilation :
- Utiliser avec précaution
- Préférer les solutions à la compilation quand possible
- Tester exhaustivement
- Documenter pourquoi c'est nécessaire
