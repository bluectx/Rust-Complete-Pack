# Pattern Matching Cheatsheet

## Syntaxe de Base

```rust
match valeur {
    pattern1 => expression1,
    pattern2 => expression2,
    _ => expression_par_defaut,
}
```

## Patterns Disponibles

```
PATTERNS
├── Littéraux          → 5, "hello", true
├── Variables          → x, y
├── Wildcard           → _
├── Ranges             → 1..=5, 'a'..='z'
├── Tuples             → (x, y), (x, ..)
├── Structs            → Point { x, y }
├── Enums              → Some(x), None
├── Guards              → x if x > 5
└── @ binding          → x @ 1..=10
```

## Exemples

```rust
// Littéraux
match x {
    1 => println!("un"),
    2 => println!("deux"),
    _ => println!("autre"),
}

// Tuples
match point {
    (0, 0) => println!("origine"),
    (x, 0) => println!("axe x: {}", x),
    (0, y) => println!("axe y: {}", y),
    (x, y) => println!("({}, {})", x, y),
}

// Enums
match option {
    Some(x) => println!("{}", x),
    None => println!("vide"),
}

// Guards
match x {
    n if n < 0 => println!("négatif"),
    n if n > 0 => println!("positif"),
    _ => println!("zéro"),
}
```

## if let

```rust
// Au lieu de:
match option {
    Some(x) => println!("{}", x),
    _ => {},
}

// Utiliser:
if let Some(x) = option {
    println!("{}", x);
}
```

## Exhaustiveness

```
Le compilateur vérifie que TOUS les cas sont couverts
```

