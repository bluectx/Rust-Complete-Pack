# Types Numériques

## Learning Objectives

- Comprendre les types entiers (i8, i16, i32, i64, i128, isize)
- Comprendre les types non-signés (u8, u16, u32, u64, u128, usize)
- Comprendre les types flottants (f32, f64)
- Connaître les plages de valeurs
- Comprendre le comportement du débordement

## Key Vocabulary

| Term | Definition |
|------|-----------|
| Entier signé | Nombre qui peut être négatif (i32) |
| Entier non-signé | Nombre positif uniquement (u32) |
| Flottant | Nombre décimal (f32, f64) |
| Overflow | Débordement quand la valeur dépasse la capacité |
| Wrapping | Comportement où la valeur "fait le tour" |

## Core Explanation

### For Absolute Beginners - C'est Comme des Boîtes de Différentes Tailles! 📦

Les nombres en Rust sont comme des **boîtes de différentes tailles** 📦:
- **Petites boîtes** (i8, u8) : Peuvent contenir peu de valeurs
- **Grandes boîtes** (i64, u64) : Peuvent contenir beaucoup de valeurs
- **Boîtes signées** (i32) : Peuvent être négatives
- **Boîtes non-signées** (u32) : Seulement positives

## Schéma Visuel - Types Numériques

```
┌─────────────────────────────────────────┐
│  📦 TYPES NUMÉRIQUES = BOÎTES 📦       │
├─────────────────────────────────────────┤
│                                         │
│  i8/u8   → Petite boîte (8 bits)       │
│  i32/u32 → Boîte moyenne (32 bits)     │
│  i64/u64 → Grande boîte (64 bits)      │
│                                         │
│  Signé (i) = Peut être négatif          │
│  Non-signé (u) = Seulement positif      │
│                                         │
│  Choisissez la bonne taille! ✅         │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Boîtes" - Les types numériques sont comme des boîtes de différentes tailles: choisissez la bonne taille pour vos valeurs!

## Code Examples

### Example 1: Types Entiers

```rust
fn main() {
    // Entiers signés (peuvent être négatifs)
    let petit: i8 = -128;      // -128 à 127
    let moyen: i32 = 1000;      // -2^31 à 2^31-1 (défaut)
    let grand: i64 = 1000000;   // -2^63 à 2^63-1
    
    // Entiers non-signés (seulement positifs)
    let petit_positif: u8 = 255;    // 0 à 255
    let moyen_positif: u32 = 2000;  // 0 à 2^32-1
    let grand_positif: u64 = 2000000; // 0 à 2^64-1
    
    println!("i8: {}, i32: {}, i64: {}", petit, moyen, grand);
    println!("u8: {}, u32: {}, u64: {}", petit_positif, moyen_positif, grand_positif);
}
```

### Example 2: Types Flottants

```rust
fn main() {
    let simple: f32 = 3.14;      // Simple précision (32 bits)
    let double: f64 = 3.14159;    // Double précision (64 bits, défaut)
    
    println!("f32: {}, f64: {}", simple, double);
    
    // Opérations mathématiques
    let somme = 5.0 + 3.0;
    let produit = 2.5 * 4.0;
    println!("Somme: {}, Produit: {}", somme, produit);
}
```

### Example 3: Overflow et Wrapping

```rust
fn main() {
    // En mode debug, overflow cause une panique
    // En mode release, wrapping (débordement silencieux)
    
    let mut x: u8 = 255;
    // x += 1;  // Panique en debug, wrapping en release
    
    // Utiliser checked_add pour gérer l'overflow
    match x.checked_add(1) {
        Some(v) => println!("Résultat: {}", v),
        None => println!("Overflow détecté!"),
    }
}
```

## Plages de Valeurs

```
TYPES ENTIERS SIGNÉS
├── i8:   -128 à 127
├── i16:  -32,768 à 32,767
├── i32:  -2,147,483,648 à 2,147,483,647 (défaut)
├── i64:  -9,223,372,036,854,775,808 à 9,223,372,036,854,775,807
└── i128: Très grand

TYPES ENTIERS NON-SIGNÉS
├── u8:   0 à 255
├── u16:  0 à 65,535
├── u32:  0 à 4,294,967,295 (défaut)
├── u64:  0 à 18,446,744,073,709,551,615
└── u128: Très grand

FLOTTANTS
├── f32: Simple précision (~7 décimales)
└── f64: Double précision (~15 décimales, défaut)
```

## Mini-exercices Rustlings

### Exercice 1: Choisir le Bon Type

```rust
fn main() {
    // TODO: Choisir le type approprié pour chaque valeur
    let age = 30;  // Quel type?
    let temperature = -5;  // Quel type?
    let population = 1000000;  // Quel type?
}
```

## Common Pitfalls

- ❌ **Mistake:** Utiliser i32 pour des valeurs qui peuvent dépasser
  ```rust
  let x: i32 = 3_000_000_000;  // ERREUR: trop grand
  ```
  ✅ **Fix:** Utiliser i64 ou u64
  ```rust
  let x: i64 = 3_000_000_000;  // CORRECT
  ```

- ❌ **Mistake:** Confondre signé et non-signé
  ```rust
  let x: u32 = -5;  // ERREUR: u32 ne peut pas être négatif
  ```
  ✅ **Fix:** Utiliser i32 pour les valeurs négatives
  ```rust
  let x: i32 = -5;  // CORRECT
  ```

## Official Resources

- [@official Rust Book - Integer Types](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types)

## Security Notes

Les overflows peuvent causer des vulnérabilités. Toujours utiliser :
- `checked_*` méthodes pour détecter les overflows
- `wrapping_*` si le wrapping est intentionnel
- `saturating_*` pour limiter aux valeurs min/max

