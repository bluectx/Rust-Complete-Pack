# Vec Avancé - Votre Meilleur Ami! 📚

## Learning Objectives

- Maîtriser Vec comme un pro (c'est facile!)
- Utiliser les méthodes avancées
- Comprendre la capacité et réallocation
- Voir des exemples COOL

## Core Explanation

### For Absolute Beginners

Vec, c'est comme une **étagère extensible**! Au début, elle a quelques cases, mais si vous ajoutez trop de livres, elle s'agrandit automatiquement! C'est **magique** et **super pratique**! ✨

## Schéma Visuel - Comment Vec Fonctionne

```
┌─────────────────────────────────────────┐
│  📚 VEC = ÉTAGÈRE EXTENSIBLE 📚         │
├─────────────────────────────────────────┤
│                                         │
│  État Initial:                          │
│  ┌───┐ ┌───┐ ┌───┐                     │
│  │ 1 │ │ 2 │ │ 3 │  (capacité: 3)      │
│  └───┘ └───┘ └───┘                     │
│                                         │
│  Après push(4):                         │
│  ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐   │
│  │ 1 │ │ 2 │ │ 3 │ │ 4 │ │ ? │ │ ? │   │
│  └───┘ └───┘ └───┘ └───┘ └───┘ └───┘   │
│         (capacité: 6 - doublée!)       │
│                                         │
│  Nouveau livre ajouté!                  │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Étagère Extensible" - Une étagère qui grandit automatiquement quand vous ajoutez des livres, comme une étagère magique qui s'agrandit!

## Code Examples

### Example 1: Vec avec Capacité (Optimisation!)

```rust
fn main() {
    // Créer Vec avec capacité initiale (plus rapide!)
    let mut nombres = Vec::with_capacity(10);
    
    // Ajouter des nombres (super facile!)
    for i in 1..=10 {
        nombres.push(i);
    }
    
    println!("Nombres: {:?}", nombres);
    println!("Capacité: {}", nombres.capacity());
}
```

### Example 2: Méthodes Cool de Vec

```rust
fn main() {
    let mut fruits = vec!["Pomme", "Banane", "Orange"];
    
    // Retirer le dernier (pop)
    let dernier = fruits.pop();
    println!("Dernier fruit: {:?}", dernier);
    
    // Insérer au milieu
    fruits.insert(1, "Kiwi");
    println!("Fruits: {:?}", fruits);
    
    // Retirer un élément
    fruits.remove(0);
    println!("Après retrait: {:?}", fruits);
    
    // Vérifier si vide
    if !fruits.is_empty() {
        println!("J'ai encore des fruits!");
    }
}
```

### Example 3: Parcourir Vec (Super Cool!)

```rust
fn main() {
    let nombres = vec![1, 2, 3, 4, 5];
    
    // Parcourir avec for (facile!)
    for nombre in &nombres {
        println!("Nombre: {}", nombre);
    }
    
    // Parcourir avec indices
    for (index, valeur) in nombres.iter().enumerate() {
        println!("Index {}: {}", index, valeur);
    }
    
    // Transformer avec map (fonctionnel!)
    let doubles: Vec<i32> = nombres.iter()
        .map(|x| x * 2)
        .collect();
    println!("Doubles: {:?}", doubles);
}
```

## Schéma - Méthodes Vec Principales

```
┌─────────────────────────────────────────┐
│  🎯 MÉTHODES VEC COOL 🎯                │
├─────────────────────────────────────────┤
│                                         │
│  push(item)      → Ajouter à la fin    │
│  pop()           → Retirer le dernier  │
│  insert(i, item) → Insérer à l'index   │
│  remove(i)       → Retirer à l'index   │
│  len()           → Taille actuelle     │
│  capacity()      → Capacité totale     │
│  is_empty()      → Vérifier si vide    │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique méthodes:** "Pousser, Retirer, Insérer, Vérifier" - Les 4 opérations principales d'une étagère!

## Official Resources

- [@official Rust Book - Vec](https://doc.rust-lang.org/book/ch08-01-vectors.html)

