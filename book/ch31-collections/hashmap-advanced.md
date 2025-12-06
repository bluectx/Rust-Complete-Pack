# HashMap Avancé - Agenda Magique! 📅

## Learning Objectives

- Maîtriser HashMap comme un agenda pro
- Utiliser les méthodes avancées
- Comprendre les collisions (c'est rare!)
- Voir des exemples COOL

## Core Explanation

### For Absolute Beginners

HashMap, c'est comme un **agenda magique**! Vous dites "Lundi" et il vous donne instantanément l'activité associée! C'est **super rapide** et **super pratique**! Vous allez adorer!

## Schéma - Comment HashMap Fonctionne

```
┌─────────────────────────────────────────┐
│  📅 HASHMAP = AGENDA MAGIQUE 📅        │
├─────────────────────────────────────────┤
│                                         │
│  Clé: "Lundi"  →  Valeur: "Réunion"    │
│  Clé: "Mardi"  →  Valeur: "Coding"     │
│  Clé: "Mercredi" → Valeur: "Rust"      │
│                                         │
│  Comment ça marche?                     │
│  1. Prendre la clé ("Lundi")            │
│  2. Calculer hash (magie!)              │
│  3. Trouver la case correspondante     │
│  4. Retourner la valeur ("Réunion")    │
│                                         │
│  C'est super rapide! ⚡                 │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Agenda Magique" - Dites le jour (clé), obtenez l'activité (valeur) instantanément!

## Code Examples

### Example 1: HashMap Basique (Super Facile!)

```rust
use std::collections::HashMap;

fn main() {
    // Créer un HashMap (agenda vide)
    let mut agenda = HashMap::new();
    
    // Ajouter des rendez-vous (clé → valeur)
    agenda.insert("Lundi", "Réunion d'équipe");
    agenda.insert("Mardi", "Coding Session");
    agenda.insert("Mercredi", "Rust Learning");
    
    // Chercher (super rapide!)
    if let Some(activite) = agenda.get("Lundi") {
        println!("Lundi: {}", activite);  // "Lundi: Réunion d'équipe"
    }
    
    // Parcourir tout l'agenda
    for (jour, activite) in &agenda {
        println!("{}: {}", jour, activite);
    }
}
```

### Example 2: Méthodes Avancées (Cool!)

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    
    // Insérer seulement si absent
    scores.entry("Alice").or_insert(0);
    scores.entry("Bob").or_insert(100);
    
    // Modifier une valeur existante
    let score = scores.entry("Alice").or_insert(0);
    *score += 50;
    
    println!("Scores: {:?}", scores);
    
    // Vérifier si contient une clé
    if scores.contains_key("Alice") {
        println!("Alice a un score!");
    }
    
    // Retirer une entrée
    scores.remove("Bob");
    println!("Après retrait: {:?}", scores);
}
```

### Example 3: Compteur avec HashMap (Super Pratique!)

```rust
use std::collections::HashMap;

fn main() {
    let texte = "rust rust rust code code";
    let mut compteur = HashMap::new();
    
    // Compter les mots (c'est facile!)
    for mot in texte.split_whitespace() {
        let count = compteur.entry(mot).or_insert(0);
        *count += 1;
    }
    
    println!("Compteur: {:?}", compteur);
    // Output: {"rust": 3, "code": 2}
    
    // Trouver le mot le plus fréquent
    let max = compteur.iter()
        .max_by_key(|(_, &count)| count);
    println!("Mot le plus fréquent: {:?}", max);
}
```

## Schéma - Méthodes HashMap Principales

```
┌─────────────────────────────────────────┐
│  🎯 MÉTHODES HASHMAP COOL 🎯            │
├─────────────────────────────────────────┤
│                                         │
│  insert(k, v)     → Ajouter clé→valeur │
│  get(k)           → Chercher (Option)  │
│  remove(k)        → Retirer une clé    │
│  contains_key(k)  → Vérifier présence    │
│  entry(k)         → Manipuler entrée   │
│  len()            → Nombre d'entrées   │
│  is_empty()       → Vérifier si vide   │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique méthodes:** "Insérer, Obtenir, Retirer, Vérifier" - Les 4 opérations principales d'un agenda!

## Official Resources

- [@official Rust Book - HashMap](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)

