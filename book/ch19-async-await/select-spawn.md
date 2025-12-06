# select! et spawn

## Learning Objectives

- Utiliser select! pour attendre plusieurs futures
- Spawner des tâches async
- Gérer la concurrence async
- Gérer les timeouts

## Code Examples

### Example 1: select!

```rust
use tokio::time::{sleep, Duration, timeout};

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = sleep(Duration::from_secs(2)) => {
            println!("Timeout");
        }
        result = async_task() => {
            println!("Tâche terminée: {:?}", result);
        }
    }
}

async fn async_task() -> i32 {
    sleep(Duration::from_secs(1)).await;
    42
}
```

### Example 2: spawn

```rust
use tokio::time::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("Tâche spawnée terminée");
        42
    });
    
    let result = handle.await.unwrap();
    println!("Résultat: {}", result);
}
```

### Example 3: Multiple Spawns

```rust
#[tokio::main]
async fn main() {
    let mut handles = vec![];
    
    for i in 0..5 {
        let handle = tokio::spawn(async move {
            println!("Tâche {}", i);
            i * 2
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let result = handle.await.unwrap();
        println!("Résultat: {}", result);
    }
}
```

### Example 4: Timeout

```rust
use tokio::time::{timeout, Duration};

#[tokio::main]
async fn main() {
    match timeout(Duration::from_secs(1), slow_task()).await {
        Ok(result) => println!("Terminé: {}", result),
        Err(_) => println!("Timeout!"),
    }
}

async fn slow_task() -> i32 {
    tokio::time::sleep(Duration::from_secs(2)).await;
    42
}
```

## Official Resources

- [Tokio select!](https://docs.rs/tokio/latest/tokio/macro.select.html)
- [Tokio spawn](https://docs.rs/tokio/latest/tokio/fn.spawn.html)

## Security Notes

- Gérer les timeouts pour éviter les blocages
- Limiter le nombre de spawns
- Surveiller les ressources
- Gérer les erreurs de spawn
