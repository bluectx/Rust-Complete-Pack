# String (Collection) - Texte Modifiable! 🎯

## Learning Objectives

- Utiliser String comme collection (c'est simple!)
- Ajouter et modifier du texte
- Comprendre l'encodage UTF-8

## Core Explanation

### For Absolute Beginners - C'est Comme un Bloc-Note! 📝

Imaginez un **bloc-note** 📝:
- **String** = Un bloc-note où vous pouvez écrire et modifier
- Vous ajoutez du texte → Il grandit!
- C'est **super pratique** pour manipuler du texte!

C'est **exactement** comme String fonctionne! C'est **super pratique**!

## Schéma Visuel - String

```
┌─────────────────────────────────────────┐
│  📝 STRING = BLOC-NOTE 📝              │
├─────────────────────────────────────────┤
│                                         │
│  String vide: ""                        │
│                                         │
│  push_str("hello"): "hello"            │
│  push(' '): "hello "                    │
│  push_str("world"): "hello world"       │
│                                         │
│  Texte modifiable! ✅                   │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Bloc-Note" - String est comme un bloc-note: vous pouvez écrire et modifier le texte, il grandit automatiquement!

## Code Examples

```rust
fn main() {
    let mut s = String::new();
    s.push_str("hello");
    s.push(' ');
    s.push_str("world");
    
    let s2 = format!("{} {}", "hello", "world");
    
    // Accès aux bytes
    for byte in s.bytes() {
        println!("{}", byte);
    }
    
    // Accès aux caractères
    for char in s.chars() {
        println!("{}", char);
    }
}
```

## Official Resources

- [@official Rust Book - String](https://doc.rust-lang.org/book/ch08-02-strings.html)

