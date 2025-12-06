# HashSet Avancé - Collection Unique! ✨

## Learning Objectives

- Maîtriser HashSet pour collections uniques
- Utiliser les opérations d'ensemble (union, intersection)
- Voir des exemples COOL

## Core Explanation

### For Absolute Beginners

HashSet, c'est comme une **collection de timbres uniques**! Chaque timbre n'apparaît qu'une fois. Si vous essayez d'ajouter un doublon, il est ignoré! C'est **super pratique** pour garder des choses uniques! ✨

## Schéma Visuel - Comment HashSet Fonctionne

```
┌─────────────────────────────────────────┐
│  ✨ HASHSET = COLLECTION UNIQUE ✨      │
├─────────────────────────────────────────┤
│                                         │
│  Ajouter "Rouge" → ✅ Ajouté            │
│  Ajouter "Bleu"  → ✅ Ajouté            │
│  Ajouter "Rouge" → ❌ Ignoré!            │
│                                         │
│  Résultat: {"Rouge", "Bleu"}           │
│  (Pas de doublons!)                    │
│                                         │
│  C'est super rapide! ⚡                 │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Collection Unique" - Comme une collection de timbres où chaque timbre n'apparaît qu'une fois, pas de doublons!

## Code Examples

### Example 1: HashSet Basique (Super Facile!)

```rust
use std::collections::HashSet;

fn main() {
    // Créer un HashSet
    let mut couleurs = HashSet::new();
    
    // Ajouter des couleurs
    couleurs.insert("Rouge");
    couleurs.insert("Bleu");
    couleurs.insert("Vert");
    couleurs.insert("Rouge");  // Ignoré! (déjà présent)
    
    println!("Couleurs uniques: {:?}", couleurs);
    // Output: {"Rouge", "Bleu", "Vert"}
    
    // Vérifier si une couleur existe
    if couleurs.contains("Rouge") {
        println!("J'ai du rouge!");
    }
}
```

### Example 2: Opérations d'Ensemble (Cool!)

```rust
use std::collections::HashSet;

fn main() {
    let set1: HashSet<_> = [1, 2, 3, 4].iter().cloned().collect();
    let set2: HashSet<_> = [3, 4, 5, 6].iter().cloned().collect();
    
    // Union (tous les éléments)
    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    println!("Union: {:?}", union);  // {1, 2, 3, 4, 5, 6}
    
    // Intersection (éléments communs)
    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    println!("Intersection: {:?}", intersection);  // {3, 4}
    
    // Différence (éléments dans set1 mais pas set2)
    let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
    println!("Différence: {:?}", difference);  // {1, 2}
}
```

### Example 3: Détecter Doublons (Super Pratique!)

```rust
use std::collections::HashSet;

fn main() {
    let mots = vec!["rust", "code", "rust", "test", "code"];
    let mut vus = HashSet::new();
    let mut doublons = Vec::new();
    
    // Trouver les doublons (c'est facile!)
    for mot in mots {
        if !vus.insert(mot) {
            doublons.push(mot);
        }
    }
    
    println!("Mots uniques: {:?}", vus);
    println!("Doublons trouvés: {:?}", doublons);
}
```

## Official Resources

- [@official Rust Book - HashSet](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)

