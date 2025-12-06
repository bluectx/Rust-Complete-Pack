# 01-01: Qu'est-ce que la Programmation?

## Learning Objectives

- Comprendre ce qu'est un programme informatique
- Comprendre la relation entre code source et exécution
- Distinguer les différents types de langages de programmation
- Comprendre le rôle du compilateur
- Appréhender les concepts de base (variables, fonctions, instructions)

## Key Vocabulary

| Term | Definition |
|------|-----------|
| Programme | Séquence d'instructions exécutables par un ordinateur |
| Code source | Texte écrit dans un langage de programmation |
| Compilateur | Programme qui transforme le code source en code machine |
| Exécution | Processus par lequel l'ordinateur exécute les instructions |
| Variable | Emplacement mémoire nommé qui stocke une valeur |
| Fonction | Bloc de code réutilisable qui effectue une tâche spécifique |

## Core Explanation

### For Absolute Beginners

Imaginez que vous donnez des instructions à un robot très obéissant mais qui ne comprend que des commandes très précises. La programmation, c'est exactement ça : écrire des instructions étape par étape pour que l'ordinateur fasse ce que vous voulez.

**Analogie du restaurant :**
- Vous (le programmeur) écrivez une recette (le code source)
- Le chef (le compilateur) transforme la recette en étapes précises
- Le cuisinier (l'ordinateur) exécute les étapes pour créer le plat (le résultat)

Un programme est une série d'instructions que l'ordinateur suit dans l'ordre. Chaque instruction est très précise et ne laisse aucune place à l'ambiguïté.

## Code Examples

### Example 1: Premier Programme Conceptuel

```rust
// Ceci est un commentaire - l'ordinateur l'ignore
// Un programme Rust commence toujours par une fonction appelée "main"

fn main() {
    // Ici, nous disons à l'ordinateur d'afficher du texte
    println!("Bonjour, monde!");
}
```

**Explanation:**

- `fn main()` : Définit la fonction principale, point d'entrée du programme
- `println!` : Instruction pour afficher du texte à l'écran
- `"Bonjour, monde!"` : Le texte à afficher
- Les accolades `{}` délimitent le bloc de code de la fonction

**Run it:**

```bash
cargo run
```

### Example 2: Variables et Instructions

```rust
fn main() {
    // Créer une variable nommée "age" qui contient le nombre 25
    let age = 25;
    
    // Afficher la valeur de la variable
    println!("J'ai {} ans", age);
    
    // Modifier la variable (nécessite "mut" pour mutable)
    let mut compteur = 0;
    compteur = compteur + 1;
    println!("Compteur: {}", compteur);
}
```

**Explanation:**

- `let age = 25;` : Crée une variable immuable nommée "age"
- `let mut compteur` : Crée une variable mutable (modifiable)
- `{}` dans println! : Placeholder pour insérer une valeur

### Example 3: Fonction Simple

```rust
// Définir une fonction qui additionne deux nombres
fn additionner(a: i32, b: i32) -> i32 {
    a + b  // Retourne le résultat (pas besoin de "return" en Rust)
}

fn main() {
    let resultat = additionner(5, 3);
    println!("5 + 3 = {}", resultat);
}
```

**Explanation:**

- `fn additionner` : Définit une nouvelle fonction
- `a: i32, b: i32` : Paramètres (nombres entiers de 32 bits)
- `-> i32` : Type de retour (entier de 32 bits)
- `a + b` : Expression retournée automatiquement

## Mini-exercices Rustlings

### Exercice 1: Fixer le Programme

```rust
// TODO: Corriger ce code pour qu'il compile et affiche "Hello, Rust!"
fn main() {
    let message = "Hello, Rust!";
    // Il manque quelque chose ici...
}
```

**Solution:**

```rust
fn main() {
    let message = "Hello, Rust!";
    println!("{}", message);
}
```

### Exercice 2: Créer une Variable

```rust
fn main() {
    // TODO: Créer une variable "nom" avec votre prénom et l'afficher
}
```

## Exercises

### Exercise 1: Premier Programme

**Level:** ⭐ Beginner

**Challenge:** Créer un programme qui affiche votre nom et votre âge.

**Expected Output:**
```
Nom: Alice
Âge: 30
```

### Exercise 2: Calcul Simple

**Level:** ⭐ Beginner

**Challenge:** Créer une fonction qui multiplie deux nombres et affiche le résultat.

**Expected:** 
```
10 * 5 = 50
```

### Exercise 3: Variables Mutiples

**Level:** ⭐⭐ Intermediate

**Challenge:** Créer trois variables (nom, ville, pays) et les afficher dans une phrase complète.

## Cheatsheet

```
PROGRAMME RUST
├── fn main() { ... }     → Point d'entrée
├── let variable = valeur; → Variable immuable
├── let mut var = val;    → Variable mutable
├── println!("{}", var);  → Affichage
└── fn nom() { ... }      → Fonction personnalisée
```

## Common Pitfalls

- ❌ **Mistake:** Oublier le point-virgule `;` à la fin des instructions
  ```rust
  let x = 5  // ERREUR: manque le ;
  ```
  ✅ **Fix:** Toujours terminer par `;`
  ```rust
  let x = 5;  // CORRECT
  ```

- ❌ **Mistake:** Essayer de modifier une variable non-mutable
  ```rust
  let x = 5;
  x = 10;  // ERREUR: x n'est pas mutable
  ```
  ✅ **Fix:** Utiliser `mut` lors de la déclaration
  ```rust
  let mut x = 5;
  x = 10;  // CORRECT
  ```

## Official Resources

- [@official Rust Book - Chapter 1](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- [@official Rust by Example - Hello World](https://doc.rust-lang.org/rust-by-example/hello.html)

## Security Notes

N/A pour ce chapitre d'introduction. Les concepts de sécurité seront introduits progressivement dans les chapitres suivants.

