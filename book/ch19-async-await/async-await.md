# async/await - Programmation Asynchrone! ⚡

## Learning Objectives

- Comprendre async/await (c'est simple!)
- Créer des fonctions async
- Utiliser .await
- Comprendre les Futures

## Core Explanation

### For Absolute Beginners - C'est Comme Faire Plusieurs Choses en Même Temps! ⚡

Imaginez que vous **faites plusieurs choses** ⚡ en même temps:
- **async** = Marquer une fonction comme asynchrone
- **await** = Attendre qu'une tâche se termine
- Pendant l'attente, d'autres tâches peuvent s'exécuter!
- C'est **super simple** et **super efficace**!

C'est **exactement** comme async/await fonctionne! C'est **super puissant**!

## Schéma Visuel - async/await

```
┌─────────────────────────────────────────┐
│  ⚡ ASYNC/AWAIT = PARALLÈLE ⚡         │
├─────────────────────────────────────────┤
│                                         │
│  async fn task1() {                    │
│    await task2()  → Attendre           │
│    // Pendant ce temps,                │
│    // d'autres tâches s'exécutent!     │
│  }                                      │
│                                         │
│  Plusieurs tâches en parallèle! ✅     │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Parallèle" - async/await permet de faire plusieurs choses en parallèle: pendant qu'une tâche attend, d'autres s'exécutent!

## Code Examples

### Example 1: Fonction Async

```rust
async fn faire_quelque_chose() -> u32 {
    42
}

#[tokio::main]
async fn main() {
    let resultat = faire_quelque_chose().await;
    println!("Résultat: {}", resultat);
}
```

## Official Resources

- [@official Rust Book - Async](https://doc.rust-lang.org/book/ch16-03-shared-state.html)

