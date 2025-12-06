# 01-05: Anatomie d'un Programme Rust

## Learning Objectives

- Comprendre la structure complète d'un programme Rust
- Connaître le rôle de chaque composant (fonctions, variables, expressions)
- Comprendre la différence entre expressions et statements
- Apprendre les conventions de nommage Rust
- Comprendre le processus de compilation

## Key Vocabulary

| Term | Definition |
|------|-----------|
| Statement | Instruction qui fait quelque chose (ne retourne pas de valeur) |
| Expression | Code qui évalue à une valeur |
| Function | Bloc de code réutilisable avec un nom |
| Block | Code délimité par des accolades `{}` |
| Compilation | Processus de transformation du code source en code machine |
| Binary | Fichier exécutable généré par la compilation |

## Core Explanation

### For Absolute Beginners

Un programme Rust est comme une recette de cuisine :
- **Fonctions** : Les étapes de la recette (couper, mélanger, cuire)
- **Variables** : Les ingrédients (nommés et stockés)
- **Expressions** : Les calculs (combien de farine ?)
- **Statements** : Les actions (mettre au four)

Le compilateur Rust lit votre code source (texte) et le transforme en code machine (0 et 1) que l'ordinateur peut exécuter. C'est comme traduire une recette en français vers des instructions que le robot cuisinier comprend.

## Code Examples

### Example 1: Structure Complète d'un Programme

```rust
// 1. Commentaires (ignorés par le compilateur)
// Ce programme calcule l'aire d'un rectangle

// 2. Fonction principale (point d'entrée)
fn main() {
    // 3. Déclaration de variables
    let largeur = 10;
    let hauteur = 5;
    
    // 4. Appel de fonction
    let aire = calculer_aire(largeur, hauteur);
    
    // 5. Affichage du résultat
    println!("L'aire du rectangle est: {}", aire);
}

// 6. Fonction personnalisée
fn calculer_aire(largeur: i32, hauteur: i32) -> i32 {
    // 7. Expression de retour (pas de point-virgule)
    largeur * hauteur
}
```

**Explanation:**

1. **Commentaires** : Documentation pour les humains
2. **fn main()** : Point d'entrée obligatoire
3. **Variables** : Stockent des valeurs nommées
4. **Appel de fonction** : Utilise une fonction définie ailleurs
5. **println!** : Macro pour afficher
6. **Fonction personnalisée** : Code réutilisable
7. **Expression de retour** : Pas de `return` ni `;` (style Rust)

### Example 2: Statements vs Expressions

```rust
fn main() {
    // STATEMENT: Fait quelque chose, ne retourne rien
    let x = 5;  // Statement (assignation)
    
    // EXPRESSION: Évalue à une valeur
    let y = {
        let a = 3;
        let b = 2;
        a + b  // Expression (pas de ;)
    };  // Le bloc entier est une expression
    
    println!("x = {}, y = {}", x, y);
    
    // if est une EXPRESSION en Rust
    let nombre = if x > 3 { 10 } else { 0 };
    println!("nombre = {}", nombre);
}
```

**Output:**
```
x = 5, y = 5
nombre = 10
```

**Explanation:**

- **Statement** : Termine par `;`, ne retourne rien
- **Expression** : Pas de `;` à la fin, retourne une valeur
- Les blocs `{}` peuvent être des expressions
- `if` est une expression (retourne une valeur)

### Example 3: Processus de Compilation

```rust
fn main() {
    let message = "Bonjour";
    println!("{}", message);
}
```

**Étapes de compilation :**

```bash
# 1. Analyse lexicale (tokenization)
# Le code est découpé en tokens
# fn, main, (, ), {, let, message, =, "Bonjour", ...

# 2. Analyse syntaxique (parsing)
# Les tokens sont organisés en arbre syntaxique
# fn main() { let message = "Bonjour"; ... }

# 3. Analyse sémantique
# Vérification des types, ownership, etc.

# 4. Génération de code
# Transformation en code machine (assembly)

# 5. Linking
# Création du binaire exécutable
```

**Exécuter avec cargo :**

```bash
cargo build        # Compile en mode debug
cargo build --release  # Compile en mode optimisé
cargo run          # Compile + exécute
```

## Conventions de Nommage Rust

```rust
// Variables et fonctions: snake_case
let ma_variable = 10;
fn ma_fonction() {}

// Types et structs: PascalCase
struct MaStructure {}
enum MonEnum {}

// Constantes: SCREAMING_SNAKE_CASE
const MA_CONSTANTE: i32 = 100;

// Modules: snake_case
mod mon_module {}

// Fichiers: snake_case.rs
// main.rs, ma_fonction.rs
```

## Structure d'un Fichier Rust

```rust
// 1. Commentaires de documentation (optionnel)
// Description du module

// 2. Imports (use statements)
use std::collections::HashMap;

// 3. Déclarations de modules (optionnel)
mod utils;

// 4. Types personnalisés
struct Point {
    x: i32,
    y: i32,
}

// 5. Implémentations
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

// 6. Fonctions
fn fonction_utilitaire() {
    // Code
}

// 7. Fonction main (seulement dans main.rs)
fn main() {
    // Point d'entrée
}
```

## Mini-exercices Rustlings

### Exercice 1: Identifier Statements et Expressions

```rust
fn main() {
    let x = 5;  // TODO: Statement ou Expression?
    let y = {
        x + 1  // TODO: Statement ou Expression?
    };
    println!("{}", y);
}
```

### Exercice 2: Corriger le Code

```rust
fn main() {
    let resultat = calculer(10, 5);
    println!("{}", resultat);
}

// TODO: Corriger cette fonction
fn calculer(a: i32, b: i32) -> i32 {
    a + b;  // Erreur: ne peut pas retourner avec ;
}
```

**Solution:**

```rust
fn calculer(a: i32, b: i32) -> i32 {
    a + b  // Pas de ; pour retourner
}
```

## Exercises

### Exercise 1: Structure Complète

**Level:** ⭐ Beginner

**Challenge:** Créer un programme avec :
- Une fonction `main()`
- Deux fonctions personnalisées
- Variables et expressions
- Affichage des résultats

### Exercise 2: Expressions Complexes

**Level:** ⭐⭐ Intermediate

**Challenge:** Utiliser des blocs comme expressions pour calculer :
- La somme de 1 à 10
- Le maximum entre trois nombres
- Une valeur conditionnelle avec `if`

## Cheatsheet

```
ANATOMIE PROGRAMME RUST
├── Commentaires        → //
├── Imports            → use
├── Types              → struct, enum
├── Implémentations    → impl
├── Fonctions          → fn
└── main()             → Point d'entrée

STATEMENTS vs EXPRESSIONS
├── Statement          → Termine par ;, ne retourne rien
└── Expression         → Pas de ;, retourne une valeur

CONVENTIONS
├── Variables          → snake_case
├── Types              → PascalCase
├── Constantes         → SCREAMING_SNAKE_CASE
└── Fichiers           → snake_case.rs
```

## Common Pitfalls

- ❌ **Mistake:** Mettre un `;` à la fin d'une expression de retour
  ```rust
  fn add(a: i32, b: i32) -> i32 {
      a + b;  // ERREUR: retourne () au lieu de i32
  }
  ```
  ✅ **Fix:** Oublier le `;` pour retourner une valeur
  ```rust
  fn add(a: i32, b: i32) -> i32 {
      a + b  // CORRECT
  }
  ```

- ❌ **Mistake:** Confondre statement et expression
  ```rust
  let x = if true { 5 };  // ERREUR: if sans else retourne ()
  ```
  ✅ **Fix:** Toujours avoir un else pour les expressions
  ```rust
  let x = if true { 5 } else { 0 };  // CORRECT
  ```

## Official Resources

- [@official Rust Book - Anatomy](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [@official Rust Book - Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

## Security Notes

Le compilateur Rust vérifie la sécurité à la compilation :
- Vérification des types (pas de type confusion)
- Vérification de l'ownership (pas de use-after-free)
- Vérification des bounds (pas de buffer overflow)
- Toutes ces vérifications se font AVANT l'exécution

