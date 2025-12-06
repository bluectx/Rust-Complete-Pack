# Traits & Bounds Cheatsheet

## Syntaxe de Base

```rust
trait NomTrait {
    fn methode(&self);
}

impl NomTrait for Type {
    fn methode(&self) {
        // Implémentation
    }
}
```

## Trait Bounds

```rust
fn fonction<T: Trait1 + Trait2>(x: T) { }

fn fonction2<T>(x: T) 
where
    T: Trait1 + Trait2,
{ }
```

## Trait Objects

```rust
let obj: Box<dyn Trait> = Box::new(instance);
```

