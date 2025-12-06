# Solution

## Approach

Plusieurs solutions possibles. La plus simple est d'utiliser seulement s2 après le move.

## Full Code - Solution 1 (Utiliser seulement s2)

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 est déplacé vers s2
    
    // Utiliser seulement s2
    println!("s2: {}", s2);
}
```

## Solution 2 (Cloner)

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();  // Clone au lieu de move
    
    println!("s1: {}", s1);
    println!("s2: {}", s2);
}
```

## Solution 3 (Borrowing)

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = &s1;  // Borrow au lieu de move
    
    println!("s1: {}", s1);
    println!("s2: {}", s2);
}
```

## Key Insights

- Move transfère l'ownership
- Clone crée une copie (coûteux pour String)
- Borrowing permet d'utiliser sans prendre ownership

## Common Mistakes

- ❌ Essayer d'utiliser une variable après move
- ✅ Utiliser borrowing ou clone si nécessaire

