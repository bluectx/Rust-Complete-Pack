# Manipulation de Chaînes - C'est Super Facile! 🎯

## Learning Objectives

- Concaténer des strings (c'est simple!)
- Formater des strings
- Parser des strings
- Utiliser les méthodes de String

## Core Explanation

### For Absolute Beginners - C'est Comme Assembler des Legos! 🧱

Imaginez assembler des **Legos** 🧱:
- **Concaténation** = Assembler plusieurs Legos ensemble
- **Formatage** = Créer des Legos avec des motifs
- **Parsing** = Séparer les Legos pour voir ce qu'ils contiennent

C'est **exactement** comme la manipulation de chaînes fonctionne! C'est **super pratique**!

## Schéma Visuel - Manipulation de Chaînes

```
┌─────────────────────────────────────────┐
│  🧱 STRINGS = ASSEMBLER LEGOS 🧱       │
├─────────────────────────────────────────┤
│                                         │
│  "hello" + ", " + "world"               │
│  ┌─────┐ ┌──┐ ┌─────┐                 │
│  │hello│ │, │ │world│ → "hello, world" │
│  └─────┘ └──┘ └─────┘                 │
│                                         │
│  format!("{} {}", "hello", "world")     │
│  → "hello world"                        │
│                                         │
│  Assembler facilement! ✅              │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Legos" - Manipuler des strings, c'est comme assembler des Legos: vous combinez des morceaux pour créer quelque chose de nouveau!

## Code Examples

### Example 1: Concaténation

```rust
fn main() {
    // Méthode 1: push_str (modifie la String)
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);
    
    // Méthode 2: + (consomme la première String)
    let s1 = String::from("hello");
    let s2 = String::from(", world");
    let s3 = s1 + &s2;  // s1 est déplacé, s2 est emprunté
    // s1 n'est plus valide ici
    println!("{}", s3);
    
    // Méthode 3: format! (ne consomme rien)
    let s4 = String::from("hello");
    let s5 = String::from("world");
    let s6 = format!("{}, {}!", s4, s5);  // s4 et s5 toujours valides
    println!("{}", s6);
}
```

### Example 2: Méthodes Utiles

```rust
fn main() {
    let s = String::from("Hello, World!");
    
    // Longueur
    println!("Longueur: {}", s.len());
    
    // Vérifications
    println!("Est vide: {}", s.is_empty());
    println!("Contient 'World': {}", s.contains("World"));
    
    // Transformation
    println!("Minuscules: {}", s.to_lowercase());
    println!("Majuscules: {}", s.to_uppercase());
    
    // Découpage
    let mots: Vec<&str> = s.split(',').collect();
    println!("Mots: {:?}", mots);
}
```

### Example 3: Formatage

```rust
fn main() {
    let nom = "Alice";
    let age = 30;
    
    // format! macro
    let message = format!("Nom: {}, Âge: {}", nom, age);
    println!("{}", message);
    
    // Avec println!
    println!("Nom: {}, Âge: {}", nom, age);
    
    // Formatage avancé
    println!("Décimal: {:.2}", 3.14159);  // 3.14
    println!("Padding: {:05}", 42);       // 00042
}
```

## Official Resources

- [@official Rust Book - String Methods](https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings)

