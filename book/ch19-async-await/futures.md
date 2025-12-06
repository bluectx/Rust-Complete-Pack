# Futures - Promesses d'Avenir! 🔮

## Learning Objectives

- Comprendre le trait Future (c'est magique!)
- Créer des futures
- Utiliser .await
- Composer des futures

## Core Explanation

### For Absolute Beginners - C'est Comme une Promesse! 🔮

Imaginez une **promesse** 🔮:
- **Future** = Une promesse d'un résultat futur
- Vous attendez (await) que la promesse se réalise
- C'est **super magique** et **super pratique**!

C'est **exactement** comme les futures fonctionnent! C'est **super puissant**!

## Schéma Visuel - Futures

```
┌─────────────────────────────────────────┐
│  🔮 FUTURE = PROMESSE 🔮              │
├─────────────────────────────────────────┤
│                                         │
│  async fn task() → Future               │
│         │                                │
│         └─> Promesse d'un résultat      │
│                                         │
│  .await → Attendre la promesse          │
│         │                                │
│         ▼ Résultat!                     │
│  ✅ Promesse réalisée!                  │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Promesse" - Une future est comme une promesse: vous attendez (await) qu'elle se réalise pour obtenir le résultat!

## Code Examples

### Example 1: Future Basique

```rust
use std::future::Future;

async fn async_function() -> i32 {
    42
}

fn main() {
    let future = async_function();
    // future est un Future, doit être await dans un contexte async
}
```

### Example 2: Implémenter Future

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MyFuture {
    value: i32,
}

impl Future for MyFuture {
    type Output = i32;
    
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.value)
    }
}

async fn use_future() {
    let future = MyFuture { value: 42 };
    let result = future.await;
    println!("{}", result);
}
```

### Example 3: Composer des Futures

```rust
use tokio::time::{sleep, Duration};

async fn task1() -> i32 {
    sleep(Duration::from_secs(1)).await;
    1
}

async fn task2() -> i32 {
    sleep(Duration::from_secs(1)).await;
    2
}

#[tokio::main]
async fn main() {
    // Exécuter en séquence
    let result1 = task1().await;
    let result2 = task2().await;
    
    // Exécuter en parallèle
    let (r1, r2) = tokio::join!(task1(), task2());
}
```

## Trait Future

```rust
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

## Official Resources

- [@official Rust Book - Futures](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Async Book - Futures](https://rust-lang.github.io/async-book/02_execution/01_chapter.html)

## Security Notes

Futures sont sûres :
- Pas de data races
- Gestion automatique des ressources
- Pas de memory leaks
