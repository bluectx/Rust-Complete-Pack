# Introduction à Async

## Learning Objectives

- Comprendre la programmation asynchrone
- Voir les avantages de async/await
- Comparer avec les threads
- Comprendre les concepts de base

## Key Vocabulary

| Term | Definition |
|------|-----------|
| Async | Programmation asynchrone |
| Future | Valeur qui sera disponible plus tard |
| await | Attendre qu'une Future se complète |
| Runtime | Environnement d'exécution async |

## Core Explanation

### For Absolute Beginners - C'est Comme Concept! 📚

Ce chapitre vous enseignera les concepts fondamentaux de manière simple et progressive.

C'est **exactement** comme ça fonctionne! C'est **super pratique**!

## Schéma Visuel - Concept

```
┌─────────────────────────────────────────┐
│  📚 CONCEPT = Concept 📚 │
├─────────────────────────────────────────┤
│                                         │
│  Concept principal                      │
│         │                                │
│         ▼ Explication                    │
│  ┌─────────────┐                        │
│  │ Concept │ → Fonctionne! ✅│
│  └─────────────┘                        │
│                                         │
│  Simple et puissant! ✅                 │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Concept" - Ce chapitre vous enseignera les concepts fondamentaux de manière simple et progressive.

## For Absolute Beginners

La programmation asynchrone, c'est comme faire plusieurs choses en même temps sans bloquer. Au lieu d'attendre qu'une tâche se termine avant de commencer la suivante, on peut démarrer plusieurs tâches et les laisser progresser en parallèle.

**Analogie :**
- **Synchrone** : Faire la vaisselle, puis le ménage, puis la cuisine (séquentiel)
- **Asynchrone** : Mettre le lave-vaisselle, puis faire le ménage pendant qu'il tourne (parallèle)

## Code Examples

### Example 1: Fonction Async

```rust
async fn faire_quelque_chose() -> u32 {
    // Simulation d'opération async
    42
}

#[tokio::main]
async fn main() {
    let resultat = faire_quelque_chose().await;
    println!("Résultat: {}", resultat);
}
```

### Example 2: Async vs Threads

```rust
use tokio::time::{sleep, Duration};

// Avec async (efficace)
async fn async_task() {
    sleep(Duration::from_secs(1)).await;
    println!("Tâche async terminée");
}

// Avec threads (plus lourd)
use std::thread;
fn thread_task() {
    thread::sleep(Duration::from_secs(1));
    println!("Tâche thread terminée");
}
```

### Example 3: Concurrence Async

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // Démarrer plusieurs tâches en parallèle
    let task1 = async {
        sleep(Duration::from_secs(1)).await;
        println!("Tâche 1");
    };
    
    let task2 = async {
        sleep(Duration::from_secs(1)).await;
        println!("Tâche 2");
    };
    
    // Exécuter en parallèle
    tokio::join!(task1, task2);
}
```

## Avantages d'Async

- **Efficacité** : Moins de threads, plus de tâches
- **Performance** : Pas de overhead de threads
- **Scalabilité** : Des milliers de connexions simultanées
- **Idiomatique** : Syntaxe claire avec async/await

## Official Resources

- [@official Rust Book - Async](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Async Book](https://rust-lang.github.io/async-book/)

## Security Notes

Async est sûr :
- Pas de data races (vérifié à la compilation)
- Gestion automatique des ressources
- Pas de race conditions dans le runtime
