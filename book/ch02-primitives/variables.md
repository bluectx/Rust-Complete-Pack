# Variables et Mutabilité

## Learning Objectives

- Comprendre la déclaration de variables avec `let`
- Distinguer variables immuables et mutables
- Comprendre l'inférence de types
- Connaître les types primitifs Rust
- Comprendre les constantes vs variables

## Key Vocabulary

| Term | Definition |
|------|-----------|
| Variable | Emplacement mémoire nommé qui stocke une valeur |
| Immutable | Ne peut pas être modifié après déclaration |
| Mutable | Peut être modifié après déclaration (avec `mut`) |
| Type inference | Capacité du compilateur à deviner le type |
| Shadowing | Redéclaration d'une variable avec le même nom |

## Core Explanation

### For Absolute Beginners - C'est Comme une Boîte avec une Étiquette! 📦

Une variable, c'est comme une **boîte avec une étiquette** 📦. En Rust, par défaut, les boîtes sont scellées (immuables) - une fois que vous y mettez quelque chose, vous ne pouvez plus le changer. Si vous voulez pouvoir modifier le contenu, vous devez explicitement demander une boîte modifiable avec `mut`.

## Schéma Visuel - Variables

```
┌─────────────────────────────────────────┐
│  📦 VARIABLES = BOÎTES ÉTIQUETÉES 📦   │
├─────────────────────────────────────────┤
│                                         │
│  let x = 5;  (Boîte scellée)            │
│  ┌─────┐                                │
│  │  5  │  ← Contenu immuable            │
│  └─────┘                                │
│                                         │
│  let mut y = 10;  (Boîte ouverte)       │
│  ┌─────┐                                │
│  │ 10 │  ← Contenu modifiable            │
│  └─────┘                                │
│                                         │
│  Boîtes étiquetées! ✅                  │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Boîtes Étiquetées" - Les variables sont comme des boîtes avec des étiquettes: immuables par défaut (scellées), mutables avec `mut` (ouvertes)!

**Pourquoi immuable par défaut ?**
- Évite les bugs accidentels
- Facilite la réflexion sur le code
- Permet des optimisations du compilateur

## Code Examples

### Example 1: Variables Immuables

```rust
fn main() {
    let x = 5;  // Variable immuable
    println!("x = {}", x);
    
    // x = 10;  // ERREUR: ne peut pas modifier une variable immuable
}
```

### Example 2: Variables Mutables

```rust
fn main() {
    let mut compteur = 0;  // Variable mutable
    compteur += 1;
    compteur += 1;
    println!("Compteur: {}", compteur);
}
```

### Example 3: Shadowing

```rust
fn main() {
    let x = 5;
    let x = x + 1;  // Shadowing: nouvelle variable avec le même nom
    let x = x * 2;
    println!("x = {}", x);  // x = 12
    
    // Shadowing permet de changer de type
    let spaces = "   ";
    let spaces = spaces.len();  // Maintenant spaces est un nombre
}
```

## Types Primitifs

### Entiers

```rust
let a: i8 = 127;      // 8 bits signé (-128 à 127)
let b: u8 = 255;      // 8 bits non-signé (0 à 255)
let c: i32 = 1000;    // 32 bits signé (défaut)
let d: u32 = 2000;    // 32 bits non-signé
let e: i64 = 1000000; // 64 bits signé
let f: u64 = 2000000; // 64 bits non-signé
let g: isize = 100;   // Taille de pointeur signé
let h: usize = 200;   // Taille de pointeur non-signé
```

### Flottants

```rust
let x: f32 = 3.14;    // 32 bits (simple précision)
let y: f64 = 3.14159; // 64 bits (double précision, défaut)
```

### Booléens et Caractères

```rust
let b: bool = true;
let c: char = 'z';    // Unicode, 4 bytes
```

## Mini-exercices Rustlings

### Exercice 1: Corriger la Mutabilité

```rust
fn main() {
    let x = 5;
    x = 10;  // TODO: Corriger pour que ça compile
    println!("x = {}", x);
}
```

## Common Pitfalls

- ❌ **Mistake:** Oublier `mut` pour modifier une variable
  ✅ **Fix:** Ajouter `mut` lors de la déclaration

- ❌ **Mistake:** Confondre shadowing et mutation
  ```rust
  let mut x = 5;
  let x = x + 1;  // Shadowing, pas mutation
  ```

## Official Resources

- [@official Rust Book - Variables](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

## Security Notes

Les variables immuables par défaut réduisent les risques de bugs de sécurité liés aux modifications accidentelles de données critiques.

