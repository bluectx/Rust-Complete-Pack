# Async/Await Cheatsheet

## Syntaxe

```rust
async fn fonction() -> Type { }

let resultat = fonction().await;
```

## Tokio Patterns

```rust
#[tokio::main]
async fn main() { }

tokio::spawn(async { });
```

## Select

```rust
tokio::select! {
    result = future1 => { },
    result = future2 => { },
}
```

