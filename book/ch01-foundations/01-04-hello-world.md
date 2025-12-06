# 01-04: Hello World

## Learning Objectives

- Créer votre premier programme Rust avec cargo
- Comprendre la structure d'un projet Rust
- Exécuter un programme avec `cargo run`
- Comprendre la fonction `main()`
- Utiliser `println!` pour afficher du texte

## Key Vocabulary

| Term | Definition |
|------|-----------|
| main() | Fonction point d'entrée de tout programme Rust |
| println! | Macro pour afficher du texte à la console |
| cargo new | Commande pour créer un nouveau projet Rust |
| Cargo.toml | Fichier de configuration du projet |
| src/ | Dossier contenant le code source |
| Macro | Fonction spéciale qui génère du code à la compilation |

## Core Explanation

### For Absolute Beginners

"Hello World" est la tradition en programmation : votre premier programme affiche simplement "Hello, World!" à l'écran. C'est comme dire "Bonjour" à l'ordinateur pour la première fois.

**Pourquoi commencer par ça ?**
- Vérifie que tout fonctionne correctement
- Apprend la structure de base d'un programme
- Donne confiance avant d'aller plus loin

En Rust, nous utilisons `cargo` pour créer et gérer nos projets. C'est comme avoir un assistant qui organise tout pour vous.

## Code Examples

### Example 1: Hello World Basique

**Créer le projet :**

```bash
cargo new hello_world
cd hello_world
```

**Structure créée :**

```
hello_world/
├── Cargo.toml
└── src/
    └── main.rs
```

**src/main.rs (créé automatiquement) :**

```rust
fn main() {
    println!("Hello, world!");
}
```

**Exécuter :**

```bash
cargo run
```

**Output:**
```
Hello, world!
```

**Explanation:**

- `fn main()` : Définit la fonction principale (point d'entrée)
- `println!` : Macro qui affiche du texte (le `!` indique une macro)
- `"Hello, world!"` : Chaîne de caractères (string) à afficher
- `cargo run` : Compile et exécute le programme

### Example 2: Hello World Personnalisé

```rust
fn main() {
    let nom = "Alice";
    let age = 30;
    
    println!("Bonjour, {}!", nom);
    println!("Vous avez {} ans.", age);
    println!("Bonjour, {nom}! Vous avez {age} ans."); // Syntaxe Rust 2021+
}
```

**Output:**
```
Bonjour, Alice!
Vous avez 30 ans.
Bonjour, Alice! Vous avez 30 ans.
```

**Explanation:**

- `let nom = "Alice";` : Crée une variable immuable
- `{}` : Placeholder pour insérer une valeur
- `{nom}` : Syntaxe Rust 2021+ pour utiliser directement le nom de variable

### Example 3: Formatage Avancé

```rust
fn main() {
    let nombre = 42;
    let decimal = 3.14159;
    
    println!("Nombre: {}", nombre);
    println!("Décimal: {:.2}", decimal);  // 2 décimales
    println!("Hexadécimal: {:x}", nombre); // Format hex
    println!("Binaire: {:b}", nombre);     // Format binaire
    println!("Padding: {:05}", nombre);    // Remplissage avec zéros
}
```

**Output:**
```
Nombre: 42
Décimal: 3.14
Hexadécimal: 2a
Binaire: 101010
Padding: 00042
```

## Structure d'un Projet Rust

### Cargo.toml

```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

[dependencies]
# Les dépendances externes vont ici
```

**Explanation:**

- `[package]` : Section de configuration du paquet
- `name` : Nom du projet
- `version` : Version du projet
- `edition` : Édition Rust utilisée
- `[dependencies]` : Bibliothèques externes (vide pour l'instant)

### src/main.rs

```rust
// Tous les programmes Rust commencent par main()
fn main() {
    // Votre code ici
}
```

## Mini-exercices Rustlings

### Exercice 1: Personnaliser le Message

```rust
fn main() {
    // TODO: Modifier pour afficher votre propre message
    println!("Hello, world!");
}
```

### Exercice 2: Afficher Plusieurs Lignes

```rust
fn main() {
    // TODO: Afficher votre nom, âge et ville sur des lignes séparées
}
```

**Solution:**

```rust
fn main() {
    println!("Nom: Alice");
    println!("Âge: 30");
    println!("Ville: Paris");
}
```

## Exercises

### Exercise 1: Premier Programme

**Level:** ⭐ Beginner

**Challenge:** Créer un projet "mon_premier_programme" qui affiche :
- Votre nom
- Votre langage de programmation préféré
- Un message de bienvenue

**Expected Output:**
```
Bonjour! Je m'appelle Alice.
J'apprends Rust.
Bienvenue dans le monde de Rust!
```

### Exercise 2: Formatage de Nombres

**Level:** ⭐⭐ Intermediate

**Challenge:** Afficher le nombre 1234.5678 avec différents formats :
- 2 décimales
- Format scientifique
- Avec séparateur de milliers

## Cheatsheet

```
HELLO WORLD RUST
├── cargo new <nom>      → Créer projet
├── cargo run            → Compiler + exécuter
├── cargo build          → Compiler seulement
└── cargo check          → Vérifier sans compiler

STRUCTURE PROJET
├── Cargo.toml           → Configuration
└── src/main.rs          → Code source

println! FORMATS
├── {}                   → Placeholder simple
├── {:.2}                → 2 décimales
├── {:x}                 → Hexadécimal
├── {:b}                 → Binaire
└── {variable}           → Syntaxe 2021+
```

## Common Pitfalls

- ❌ **Mistake:** Oublier le point d'exclamation dans `println!`
  ```rust
  println("Hello");  // ERREUR: println n'existe pas
  ```
  ✅ **Fix:** Toujours utiliser `println!` avec le `!`
  ```rust
  println!("Hello");  // CORRECT
  ```

- ❌ **Mistake:** Essayer d'exécuter main.rs directement
  ```bash
  ./src/main.rs  # Ne fonctionne pas
  ```
  ✅ **Fix:** Utiliser cargo
  ```bash
  cargo run  # CORRECT
  ```

- ❌ **Mistake:** Oublier les guillemets autour des strings
  ```rust
  println!(Hello);  // ERREUR
  ```
  ✅ **Fix:** Toujours utiliser des guillemets
  ```rust
  println!("Hello");  // CORRECT
  ```

## Official Resources

- [@official Rust Book - Hello World](https://doc.rust-lang.org/book/ch01-02-hello-world.html)
- [@official Rust by Example - Hello World](https://doc.rust-lang.org/rust-by-example/hello.html)

## Security Notes

N/A pour ce chapitre d'introduction. Les concepts de sécurité seront introduits dans les chapitres suivants. Notez que `println!` est sûr et ne peut pas causer de buffer overflow (contrairement à `printf` en C).

