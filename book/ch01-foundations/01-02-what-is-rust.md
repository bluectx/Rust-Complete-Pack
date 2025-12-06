# 01-02: Qu'est-ce que Rust?

## Learning Objectives

- Comprendre l'histoire et les origines de Rust
- Connaître les principaux cas d'usage de Rust
- Comprendre les avantages de Rust (sécurité, performance, concurrence)
- Comparer Rust avec d'autres langages (C, C++, Go, Python)
- Appréhender l'écosystème Rust (crates, cargo, communauté)

## Key Vocabulary

| Term | Definition |
|------|-----------|
| Rust | Langage de programmation système créé par Mozilla |
| Borrow checker | Système qui garantit la sécurité mémoire à la compilation |
| Zero-cost abstractions | Abstractions qui n'ont pas de coût en performance |
| Ownership | Système de propriété unique des valeurs en mémoire |
| Cargo | Gestionnaire de paquets et outil de build pour Rust |
| Crate | Unité de compilation et de distribution en Rust |

## Core Explanation

### For Absolute Beginners

Rust est un langage de programmation créé pour résoudre un problème majeur : comment écrire du code rapide ET sûr en même temps ?

**Analogie de la construction :**
- **C/C++** : Comme construire sans garde-corps - très rapide mais dangereux
- **Python/Java** : Comme construire avec beaucoup de sécurité mais plus lent
- **Rust** : Comme construire avec des garde-corps automatiques qui n'empêchent pas la vitesse

Rust a été créé par Mozilla en 2010 pour développer le moteur de rendu Firefox (Servo). L'objectif était de créer un langage qui :
1. Soit aussi rapide que C/C++
2. Soit aussi sûr que les langages modernes
3. Gère la concurrence sans bugs

## Code Examples

### Example 1: Sécurité Mémoire Automatique

```rust
fn main() {
    // En Rust, le compilateur empêche les erreurs mémoire courantes
    let mut vec = vec![1, 2, 3];
    
    // Cette ligne serait dangereuse en C/C++ mais Rust la détecte
    // let reference = &vec[0];
    // vec.push(4);  // ERREUR: ne peut pas muter pendant qu'une référence existe
    
    // Solution Rust: gérer explicitement les références
    {
        let reference = &vec[0];
        println!("Premier élément: {}", reference);
    } // reference n'existe plus ici
    vec.push(4); // Maintenant c'est sûr
}
```

**Explanation:**

- Rust empêche les "use-after-free" et "double-free" à la compilation
- Le borrow checker vérifie que les références sont valides
- Pas besoin de garbage collector (comme Java/Go) ni de gestion manuelle (comme C/C++)

### Example 2: Performance Native

```rust
// Ce code compile en code machine optimisé, aussi rapide que du C
fn calcul_intensif(n: u64) -> u64 {
    let mut somme = 0;
    for i in 0..n {
        somme += i * i;
    }
    somme
}

fn main() {
    let resultat = calcul_intensif(1_000_000);
    println!("Résultat: {}", resultat);
}
```

**Explanation:**

- Rust compile en code machine natif (pas d'interpréteur)
- Performance comparable à C/C++
- Optimisations automatiques par le compilateur

### Example 3: Concurrence Sûre

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Bonjour depuis un thread!");
    });
    
    handle.join().unwrap();
    println!("Thread terminé");
}
```

**Explanation:**

- Rust garantit l'absence de data races à la compilation
- Pas besoin de verrous explicites dans beaucoup de cas
- Le système de types empêche les accès concurrents dangereux

## Comparaisons avec Autres Langages

### Rust vs C/C++

| Aspect | C/C++ | Rust |
|--------|-------|------|
| Sécurité mémoire | Manuelle, erreurs fréquentes | Automatique, vérifiée à la compilation |
| Performance | Très rapide | Aussi rapide |
| Gestion mémoire | Manuelle (malloc/free) | Automatique (ownership) |
| Concurrence | Difficile, propice aux erreurs | Sûre par design |

### Rust vs Go

| Aspect | Go | Rust |
|--------|----|------|
| Garbage collector | Oui (pause possible) | Non (zero-cost) |
| Performance | Très bonne | Excellente |
| Simplicité | Plus simple | Plus complexe |
| Cas d'usage | Services web, outils | Systèmes, performance critique |

### Rust vs Python

| Aspect | Python | Rust |
|--------|--------|------|
| Vitesse d'exécution | Lente (interprété) | Très rapide (compilé) |
| Facilité d'apprentissage | Très facile | Plus difficile |
| Cas d'usage | Scripts, data science | Systèmes, performance |

## Cas d'Usage de Rust

1. **Systèmes d'exploitation** : Linux kernel 6.1+ inclut du code Rust
2. **Moteurs de navigateur** : Firefox (Servo), Chrome (certains composants)
3. **Infrastructure cloud** : Cloudflare, AWS, Microsoft Azure
4. **Blockchain** : Solana, Polkadot, Ethereum clients
5. **Outils DevOps** : ripgrep, fd, bat, exa
6. **Jeux vidéo** : Moteurs de jeu, outils de développement
7. **WebAssembly** : Compilation vers WASM pour le web

## Mini-exercices Rustlings

### Exercice 1: Comprendre l'Ownership

```rust
fn main() {
    let s1 = String::from("hello");
    // TODO: Que se passe-t-il si on essaie d'utiliser s1 après cette ligne?
    let s2 = s1;
    // println!("{}", s1);  // Décommentez et voyez l'erreur
}
```

## Exercises

### Exercise 1: Comparaison de Performance

**Level:** ⭐⭐ Intermediate

**Challenge:** Écrire un programme qui calcule la somme des carrés de 1 à 1_000_000 et mesure le temps d'exécution.

### Exercise 2: Recherche d'Information

**Level:** ⭐ Beginner

**Challenge:** Rechercher et lister 5 projets open-source majeurs écrits en Rust.

## Cheatsheet

```
RUST EN BREF
├── Créé par: Mozilla (2010)
├── Philosophie: Sécurité + Performance
├── Points forts:
│   ├── Sécurité mémoire (compile-time)
│   ├── Performance native (comme C/C++)
│   ├── Concurrence sûre
│   └── Zero-cost abstractions
└── Cas d'usage: Systèmes, performance, infrastructure
```

## Common Pitfalls

- ❌ **Mistake:** Penser que Rust est trop complexe pour débuter
  ✅ **Fix:** Rust a une courbe d'apprentissage, mais la sécurité en vaut la peine

- ❌ **Mistake:** Comparer Rust uniquement à un langage
  ✅ **Fix:** Rust combine les avantages de plusieurs langages

## Official Resources

- [@official Rust Book - Foreword](https://doc.rust-lang.org/book/foreword.html)
- [@official Rust Website](https://www.rust-lang.org/)
- [@official Rust by Example](https://doc.rust-lang.org/rust-by-example/)

## Security Notes

Rust est conçu pour la sécurité :
- Pas de buffer overflows (vérifiés à la compilation)
- Pas de use-after-free (ownership system)
- Pas de data races (Send/Sync traits)
- Unsafe code explicitement marqué (blocs `unsafe`)

