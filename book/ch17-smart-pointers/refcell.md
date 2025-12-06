# RefCell - Interior Mutability! 🎯

## Learning Objectives

- Utiliser RefCell pour interior mutability (c'est puissant!)
- Comprendre les borrows à l'exécution
- Gérer les panics de borrow
- Voir les cas d'usage

## Core Explanation

### For Absolute Beginners - C'est Comme un Cornichon! 🥒

Imaginez un **cornichon** 🥒 dans un bocal:
- **RefCell** = Le bocal qui permet de modifier le cornichon même si le bocal est immuable
- Vérification à l'exécution (pas à la compilation!)
- Si vous violez les règles → PANIC!

C'est **exactement** comme RefCell fonctionne! C'est **super flexible** mais **attention**!

## Schéma Visuel - RefCell

```
┌─────────────────────────────────────────┐
│  🥒 REFCELL = CORNICHON 🥒            │
├─────────────────────────────────────────┤
│                                         │
│  let data = RefCell::new(5);           │
│  ┌─────────────┐                        │
│  │ Bocal       │ (immuable)            │
│  │ ┌─────────┐ │                        │
│  │ │ Cornichon│ │ (modifiable!)         │
│  │ └─────────┘ │                        │
│  └─────────────┘                        │
│                                         │
│  borrow_mut() → Modifier               │
│  Vérification à l'exécution! ⚠️        │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Cornichon" - RefCell est comme un cornichon dans un bocal: le bocal est immuable mais vous pouvez modifier le cornichon à l'intérieur, avec vérification à l'exécution!

## Code Examples

### Example 1: RefCell Basique

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    
    {
        let mut borrow = data.borrow_mut();
        *borrow += 1;
    }  // borrow libéré ici
    
    let read = data.borrow();
    println!("{}", *read);
}
```

### Example 2: Interior Mutability

```rust
use std::cell::RefCell;

struct MockMessenger {
    messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn send(&self, msg: &str) {
        // self n'est pas mutable, mais on peut modifier messages
        self.messages.borrow_mut().push(msg.to_string());
    }
}

fn main() {
    let messenger = MockMessenger {
        messages: RefCell::new(Vec::new()),
    };
    
    messenger.send("Hello");
    messenger.send("World");
    
    println!("Messages: {:?}", messenger.messages.borrow());
}
```

### Example 3: Panic de Borrow

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    
    let _borrow1 = data.borrow_mut();
    // let _borrow2 = data.borrow();  // PANIC! Déjà emprunté mutablement
    
    // Solution: libérer borrow1 d'abord
}
```

## Règles de RefCell

- **borrow()** : Emprunt immuable (peut y en avoir plusieurs)
- **borrow_mut()** : Emprunt mutable (un seul à la fois)
- **Vérification à l'exécution** : Panique si règles violées
- **Single-threaded** : Pas thread-safe (utiliser Mutex pour multi-thread)

## Official Resources

- [@official Rust Book - RefCell](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)

## Security Notes

RefCell vérifie à l'exécution :
- Panique si règles de borrowing violées
- Pas de vérification à la compilation
- Utiliser avec précaution
- Préférer les vérifications à la compilation quand possible
