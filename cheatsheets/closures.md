# Closures Cheatsheet

## Syntaxe

```rust
|param| expression
|param| { statements }
|param: Type| -> Type { body }
```

## Fn Traits

```
Fn      → Peut être appelé plusieurs fois, ne capture pas mutablement
FnMut   → Peut être appelé plusieurs fois, capture mutablement
FnOnce  → Peut être appelé une fois, prend ownership
```

## Move

```rust
let closure = move |x| x + captured_var;
```

