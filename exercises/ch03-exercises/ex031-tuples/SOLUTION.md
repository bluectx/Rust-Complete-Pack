# Solution

## Approach

Utiliser un tuple pour regrouper plusieurs valeurs de types différents et les retourner depuis une fonction.

## Full Code

```rust
fn obtenir_infos() -> (String, u32, String) {
    ("Alice".to_string(), 30, "Paris".to_string())
}

fn main() {
    let (nom, age, ville) = obtenir_infos();
    println!("Nom: {}, Âge: {}, Ville: {}", nom, age, ville);
}
```

## Key Insights

- Les tuples permettent de regrouper plusieurs valeurs
- La déstructuration permet d'extraire les valeurs facilement
- Les tuples sont utiles pour retourner plusieurs valeurs

## Common Mistakes

- ❌ Oublier les types dans la signature de fonction
- ✅ Toujours spécifier les types: `(String, u32, String)`

