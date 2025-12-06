# Borrowing (Emprunt)

## Learning Objectives

- Comprendre le concept de borrowing
- Utiliser les références (`&`) et références mutables (`&mut`)
- Comprendre la règle XOR (exclusive ou)
- Distinguer borrowing et ownership

## Key Vocabulary

| Term | Definition |
|------|-----------|
| Borrowing | Emprunt temporaire d'une valeur sans prendre ownership |
| Reference | Pointeur vers une valeur (`&T`) |
| Mutable reference | Référence mutable (`&mut T`) |
| Règle XOR | Pas de références mutables en même temps qu'autres références |

## Core Explanation

### For Absolute Beginners - C'est Comme Emprunter un Livre! 📖

Le borrowing, c'est comme **emprunter un livre** 📖 à la bibliothèque :
- Vous pouvez **lire** le livre (référence `&`)
- Vous pouvez **modifier** le livre si vous avez la permission (référence `&mut`)
- Mais vous devez le **rendre** (la référence expire)
- Et vous ne pouvez pas le modifier pendant que quelqu'un d'autre le lit

## Schéma Visuel - Borrowing

```
┌─────────────────────────────────────────┐
│  📖 BORROWING = EMPRUNTER LIVRE 📖     │
├─────────────────────────────────────────┤
│                                         │
│  Livre (propriétaire: Alice)            │
│         │                                │
│         ├─> &livre → Bob lit (plusieurs OK) │
│         └─> &mut livre → Charlie modifie (un seul) │
│                                         │
│  Règle XOR:                              │
│  - Plusieurs & OU un seul &mut          │
│                                         │
│  Emprunt sûr! ✅                        │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Emprunter Livre" - Le borrowing est comme emprunter un livre: vous pouvez lire (&) ou modifier (&mut), mais avec des règles strictes!

## Code Examples

### Example 1: Référence Immutable

```rust
fn calculer_longueur(s: &String) -> usize {
    s.len()  // Emprunte s sans prendre ownership
}

fn main() {
    let s1 = String::from("hello");
    let len = calculer_longueur(&s1);  // Passe une référence
    println!("La longueur de '{}' est {}.", s1, len);
    // s1 est toujours valide car on a seulement emprunté
}
```

**Explanation:**

- `&String` : Type de référence vers String
- `&s1` : Crée une référence vers s1
- `s1` reste valide après l'appel

### Example 2: Référence Mutable

```rust
fn modifier(s: &mut String) {
    s.push_str(", world!");
}

fn main() {
    let mut s = String::from("hello");
    modifier(&mut s);  // Passe une référence mutable
    println!("{}", s);  // "hello, world!"
}
```

### Example 3: Règle XOR

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;      // OK
    let r2 = &s;      // OK: plusieurs références immutables
    // let r3 = &mut s;  // ERREUR: ne peut pas avoir référence mutable
    //                  // en même temps que références immutables
    
    println!("{}, {}", r1, r2);
    
    // r1 et r2 ne sont plus utilisées après ici
    let r3 = &mut s;  // OK maintenant
    r3.push_str("!");
}
```

**Règle XOR :**
- Soit **une seule** référence mutable (`&mut`)
- Soit **plusieurs** références immutables (`&`)
- **Jamais les deux en même temps**

## Comparaisons avec C/C++

### En C++ (Références Non-Sécurisées)

```cpp
// C++: Références mais pas de vérification à la compilation
string s = "hello";
string& ref = s;
// Pas de protection contre use-after-free
// Pas de protection contre data races
```

### En Rust (Références Sécurisées)

```rust
// Rust: Références vérifiées à la compilation
let s = String::from("hello");
let ref = &s;
// Impossible d'avoir use-after-free
// Impossible d'avoir data races (règle XOR)
```

## Diagramme de Borrowing

```
OWNERSHIP
┌─────────┐
│ s       │───> String("hello")
└─────────┘

BORROWING
┌─────────┐
│ s       │───> String("hello")
└─────────┘      ▲
                 │
┌─────────┐      │
│ ref     │──────┘  (référence, pas ownership)
└─────────┘

s reste propriétaire
ref est juste un emprunt
```

## Mini-exercices Rustlings

### Exercice 1: Corriger le Borrowing

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s;  // TODO: Corriger cette erreur
    println!("{}", r1);
}
```

**Solution:**

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    println!("{}", r1);  // Utiliser r1 d'abord
    let r2 = &mut s;     // Maintenant c'est OK
}
```

## Exercises

### Exercise 1: Fonction avec Borrowing

**Level:** ⭐⭐ Intermediate

**Challenge:** Créer une fonction qui prend une référence vers String et retourne sa longueur, sans prendre ownership.

### Exercise 2: Règle XOR

**Level:** ⭐⭐⭐ Advanced

**Challenge:** Expliquer pourquoi ce code ne compile pas et le corriger :

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s;
    println!("{}", r1);
}
```

## Cheatsheet

```
BORROWING
├── &T          → Référence immutable
├── &mut T      → Référence mutable
└── Règle XOR   → Une seule &mut OU plusieurs &

RÈGLES
├── Références doivent toujours être valides
├── Pas de référence mutable + immutables en même temps
└── Pas de références mutables multiples
```

## Common Pitfalls

- ❌ **Mistake:** Violer la règle XOR
  ```rust
  let mut s = String::from("hello");
  let r1 = &s;
  let r2 = &mut s;  // ERREUR
  ```
  ✅ **Fix:** Utiliser les références dans des scopes séparés

- ❌ **Mistake:** Retourner une référence vers une valeur locale
  ```rust
  fn dangle() -> &String {
      let s = String::from("hello");
      &s  // ERREUR: s n'existe plus après la fonction
  }
  ```
  ✅ **Fix:** Retourner la valeur directement (ownership)

## Official Resources

- [@official Rust Book - References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

## Security Notes

Le borrowing empêche :
- **Use-after-free** : Références vérifiées à la compilation
- **Data races** : Règle XOR empêche les accès concurrents dangereux
- **Invalid references** : Le compilateur garantit la validité

