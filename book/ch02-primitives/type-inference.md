# Inférence de Types

## Learning Objectives

- Comprendre comment Rust infère les types
- Savoir quand spécifier explicitement le type
- Comprendre les annotations de type
- Connaître les cas où l'inférence échoue

## Key Vocabulary

| Term | Definition |
|------|-----------|
| Type inference | Capacité du compilateur à deviner le type |
| Annotation de type | Spécification explicite du type |
| Ambiguïté | Cas où le compilateur ne peut pas deviner |

## Core Explanation

### For Absolute Beginners - C'est Comme un Détective! 🕵️

L'inférence de types, c'est comme un **détective** 🕵️ qui devine ce que vous voulez dire :
- Vous dites "5" → Le détective devine que c'est un nombre entier
- Vous dites "5.0" → Le détective devine que c'est un nombre décimal
- Parfois, le détective a besoin d'aide → Vous devez préciser explicitement

Rust est très intelligent et peut souvent deviner le type, mais parfois vous devez l'aider.

## Schéma Visuel - Inférence de Types

```
┌─────────────────────────────────────────┐
│  🕵️ INFÉRENCE = DÉTECTIVE 🕵️          │
├─────────────────────────────────────────┤
│                                         │
│  Vous: "5"                               │
│         │                                │
│         ▼ Détective devine               │
│  Rust: "C'est un i32!"                  │
│                                         │
│  Vous: "5.0"                             │
│         │                                │
│         ▼ Détective devine               │
│  Rust: "C'est un f64!"                  │
│                                         │
│  Détective intelligent! ✅              │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Détective" - L'inférence de types est comme un détective: Rust devine le type automatiquement, mais parfois vous devez l'aider!

## Code Examples

### Example 1: Inférence Automatique

```rust
fn main() {
    // Rust devine automatiquement les types
    let x = 5;           // i32 (défaut pour entiers)
    let y = 5.0;        // f64 (défaut pour flottants)
    let z = true;       // bool
    let s = "hello";    // &str
    
    println!("x: {}, y: {}, z: {}, s: {}", x, y, z, s);
    
    // Le compilateur connaît les types même si on ne les écrit pas
    let somme = x + 10;  // Rust sait que x est i32
}
```

### Example 2: Annotation Explicite

```rust
fn main() {
    // Parfois, on doit spécifier le type explicitement
    let x: i64 = 5;        // Forcer i64 au lieu de i32
    let y: f32 = 3.14;     // Forcer f32 au lieu de f64
    let z: u8 = 255;       // Forcer u8
    
    // Utile pour les collections
    let vec: Vec<i32> = Vec::new();  // Sans annotation, Rust ne sait pas quel type
    let vec2 = vec![1, 2, 3];        // Ici, Rust peut deviner: Vec<i32>
}
```

### Example 3: Cas d'Ambiguïté

```rust
fn main() {
    // ERREUR: Rust ne peut pas deviner le type
    // let x = Vec::new();  // Quel type de Vec?
    
    // SOLUTION 1: Annotation explicite
    let x: Vec<i32> = Vec::new();
    
    // SOLUTION 2: Utiliser une méthode qui donne un indice
    let y = Vec::<i32>::new();
    
    // SOLUTION 3: Ajouter un élément pour donner un indice
    let mut z = Vec::new();
    z.push(5);  // Maintenant Rust sait que z est Vec<i32>
}
```

### Example 4: Inférence dans les Fonctions

```rust
// Le type de retour doit être explicite
fn additionner(a: i32, b: i32) -> i32 {
    a + b  // Rust infère que c'est i32 (correspond au type de retour)
}

fn main() {
    let resultat = additionner(5, 3);
    // Rust infère que resultat est i32 (d'après le type de retour)
    println!("Résultat: {}", resultat);
}
```

## Règles d'Inférence

```
RUST PEUT INFÉRER:
├── Types primitifs (i32, f64, bool, char)
├── Types de collections (si on donne un indice)
├── Types de retour (si explicite dans la signature)
└── Types de variables locales

RUST NE PEUT PAS INFÉRER:
├── Types de paramètres de fonction (toujours explicites)
├── Types de retour (toujours explicites)
├── Collections vides sans contexte
└── Types génériques sans contexte
```

## Mini-exercices Rustlings

### Exercice 1: Corriger l'Ambiguïté

```rust
fn main() {
    // TODO: Corriger pour que ça compile
    let vec = Vec::new();
    vec.push(5);
}
```

**Solution:**

```rust
fn main() {
    let mut vec = Vec::new();
    vec.push(5);  // Maintenant Rust sait que vec est Vec<i32>
}
```

## Common Pitfalls

- ❌ **Mistake:** Oublier que les collections vides ont besoin d'annotation
  ```rust
  let vec = Vec::new();  // ERREUR: type ambigu
  ```
  ✅ **Fix:** Annoter ou donner un indice
  ```rust
  let vec: Vec<i32> = Vec::new();  // CORRECT
  ```

- ❌ **Mistake:** Penser que l'inférence fonctionne partout
  ```rust
  fn ma_fonction(x) {  // ERREUR: types de paramètres toujours explicites
  ```
  ✅ **Fix:** Toujours annoter les paramètres
  ```rust
  fn ma_fonction(x: i32) {  // CORRECT
  ```

## Official Resources

- [@official Rust Book - Type Inference](https://doc.rust-lang.org/book/ch03-02-data-types.html)

## Security Notes

L'inférence de types est sûre. Le compilateur vérifie toujours les types, même s'ils sont inférés. Aucun risque de sécurité lié à l'inférence.

