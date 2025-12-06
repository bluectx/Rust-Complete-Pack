# Paramètres et Valeurs de Retour - C'est Super Facile! 🎯

## Learning Objectives

- Déclarer des fonctions avec paramètres typés (c'est simple!)
- Comprendre les types de retour
- Retourner plusieurs valeurs avec tuples
- Utiliser les références comme paramètres

## Core Explanation

### For Absolute Beginners - C'est Comme Donner et Recevoir! 🎁

Imaginez que vous **donnez** 🎁 quelque chose et **recevez** un résultat:
- **Paramètres** = Ce que vous donnez (ingrédients)
- **Retour** = Ce que vous recevez (résultat)
- Vous pouvez donner plusieurs choses et recevoir plusieurs résultats!

C'est **exactement** comme les paramètres et retours fonctionnent! C'est **super logique**!

## Schéma Visuel - Paramètres & Retour

```
┌─────────────────────────────────────────┐
│  🎁 PARAMÈTRES & RETOUR 🎁            │
├─────────────────────────────────────────┤
│                                         │
│  fn calculer(a: i32, b: i32)            │
│         │    └─┬─┘                       │
│         │      └─> Paramètres (donner) │
│         │                                │
│         └─> → (i32, i32)                │
│             └─> Retour (recevoir)        │
│                                         │
│  Donner → Recevoir! ✅                  │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Donner/Recevoir" - Les paramètres sont ce que vous donnez, le retour est ce que vous recevez!

## Code Examples

### Example 1: Paramètres Typés

```rust
fn additionner(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let resultat = additionner(5, 3);
    println!("5 + 3 = {}", resultat);
}
```

### Example 2: Retour Multiple avec Tuple

```rust
fn calculer(a: i32, b: i32) -> (i32, i32, i32) {
    (a + b, a - b, a * b)
}

fn main() {
    let (somme, difference, produit) = calculer(10, 5);
    println!("Somme: {}, Différence: {}, Produit: {}", 
             somme, difference, produit);
}
```

### Example 3: Paramètres par Référence

```rust
fn afficher_longueur(s: &String) {
    println!("Longueur: {}", s.len());
}

fn modifier(s: &mut String) {
    s.push_str("!");
}

fn main() {
    let s = String::from("hello");
    afficher_longueur(&s);  // Emprunte
    
    let mut s2 = String::from("world");
    modifier(&mut s2);  // Emprunte mutable
    println!("{}", s2);
}
```

## Official Resources

- [@official Rust Book - Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

