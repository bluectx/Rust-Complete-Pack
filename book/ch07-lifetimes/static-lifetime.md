# Lifetime 'static - Pour Toujours! ♾️

## Learning Objectives

- Comprendre 'static lifetime (c'est simple!)
- Utiliser &'static str pour les string literals
- Distinguer 'static et durée de vie du programme

## Core Explanation

### For Absolute Beginners - C'est Comme Pour Toujours! ♾️

Imaginez quelque chose qui dure **pour toujours** ♾️:
- **'static** = Dure pour toute la durée du programme
- Les string literals ont 'static lifetime
- C'est **super simple** et **super pratique**!

C'est **exactement** comme 'static fonctionne! C'est **super logique**!

## Schéma Visuel - 'static

```
┌─────────────────────────────────────────┐
│  ♾️ 'STATIC = POUR TOUJOURS ♾️        │
├─────────────────────────────────────────┤
│                                         │
│  &'static str = "hello"                │
│         │                                │
│         └─> Dure pour TOUJOURS!        │
│                                         │
│  Valide pour toute la durée du programme! ✅│
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Pour Toujours" - 'static dure pour toujours, comme les string literals qui vivent toute la durée du programme!

## Code Examples

### Example 1: 'static Lifetime

```rust
fn main() {
    // String literals ont le lifetime 'static
    let s: &'static str = "Je vis pour toute la durée du programme";
    
    // 'static signifie que la référence est valide pour toute la durée du programme
    println!("{}", s);
}

fn retourner_static() -> &'static str {
    "Ceci est 'static"
}
```

### Example 2: 'static vs Autres Lifetimes

```rust
fn main() {
    // 'static: valide pour toute la durée du programme
    let s1: &'static str = "hello";
    
    // Lifetime local: valide seulement dans ce scope
    let s2 = String::from("world");
    let s3: &str = &s2;  // Lifetime de s2
    
    // s1 peut être utilisé partout
    // s3 ne peut être utilisé que tant que s2 existe
}
```

## Official Resources

- [@official Rust Book - 'static](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#the-static-lifetime)

