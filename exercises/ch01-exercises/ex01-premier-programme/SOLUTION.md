# Solution

## Approach

Utiliser `println!` pour afficher chaque information sur une ligne séparée.

## Full Code

```rust
fn main() {
    println!("Nom: Alice");
    println!("Âge: 30");
    println!("Langage préféré: Rust");
}
```

## Key Insights

- `println!` est une macro pour afficher du texte
- Chaque `println!` crée une nouvelle ligne
- Les chaînes de caractères sont entre guillemets doubles

## Optimization

Pour rendre le programme plus flexible, on pourrait utiliser des variables :

```rust
fn main() {
    let nom = "Alice";
    let age = 30;
    let langage = "Rust";
    
    println!("Nom: {}", nom);
    println!("Âge: {}", age);
    println!("Langage préféré: {}", langage);
}
```

## Common Mistakes

- ❌ Oublier les guillemets autour des strings
- ✅ Toujours utiliser `"..."` pour les chaînes

- ❌ Oublier le `!` dans `println!`
- ✅ `println!` est une macro, le `!` est obligatoire

