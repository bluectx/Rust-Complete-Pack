# Ownership & Lifetimes Cheatsheet

## Règles d'Ownership

```
1. Chaque valeur a UN SEUL propriétaire
2. Un seul propriétaire à la fois
3. Libération automatique en sortie de portée
```

## Move vs Copy

```
TYPES COPY (copie automatique)
├── i8, i16, i32, i64, i128, isize
├── u8, u16, u32, u64, u128, usize
├── f32, f64
├── bool
├── char
└── Tuples de types Copy

TYPES MOVE (transfert d'ownership)
├── String
├── Vec<T>
├── HashMap<K, V>
└── Tous les types non-Copy
```

## Borrowing (Règle XOR)

```
┌─────────────────────────────────┐
│ RÈGLE XOR                       │
├─────────────────────────────────┤
│ Soit:                           │
│   • Une seule &mut T            │
│ OU:                             │
│   • Plusieurs &T                │
│                                 │
│ JAMAIS les deux en même temps!  │
└─────────────────────────────────┘
```

## Diagramme Mémoire

```
OWNERSHIP
┌─────────┐
│ s       │───> String("hello")
└─────────┘

MOVE
┌─────────┐
│ s1      │───> String("hello")
└─────────┘      ▲
                 │
┌─────────┐      │
│ s2      │──────┘ (s1 invalide)
└─────────┘

BORROWING
┌─────────┐
│ s       │───> String("hello")
└─────────┘      ▲
                 │
┌─────────┐      │
│ ref     │──────┘ (s toujours propriétaire)
└─────────┘
```

## Lifetimes

```rust
// Lifetime de base
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 'static: durée de vie du programme
let s: &'static str = "hello";

// Elision: le compilateur devine
fn first(s: &str) -> &str {  // Équivaut à &'a str
    &s[0..1]
}
```

## Mnemonics

- **RAII**: Resource Acquisition Is Initialization
- **XOR**: Exclusive OR (une seule référence mutable OU plusieurs immutables)
- **Move = Transfert**, Copy = Duplication

## Common Patterns

```rust
// Pattern 1: Prendre ownership
fn prendre(s: String) { }

// Pattern 2: Emprunter
fn emprunter(s: &String) { }

// Pattern 3: Emprunter mutable
fn modifier(s: &mut String) { }

// Pattern 4: Retourner ownership
fn creer() -> String {
    String::from("hello")
}
```

