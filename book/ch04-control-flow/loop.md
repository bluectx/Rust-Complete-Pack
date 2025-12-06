# Boucle loop - Infinie! ♾️

## Learning Objectives

- Utiliser la boucle loop infinie (c'est puissant!)
- Utiliser break pour sortir
- Utiliser continue pour sauter une itération
- Utiliser les labels pour les boucles imbriquées

## Core Explanation

### For Absolute Beginners - C'est Comme une Boucle Infinie! ♾️

Imaginez une **boucle infinie** ♾️:
- **loop** = Répéter pour toujours (jusqu'à ce que vous disiez stop!)
- **break** = Dire stop (sortir de la boucle)
- **continue** = Passer au tour suivant
- C'est **super pratique** pour les boucles infinies!

C'est **exactement** comme loop fonctionne! C'est **super flexible**!

## Schéma Visuel - Loop

```
┌─────────────────────────────────────────┐
│  ♾️ LOOP = BOUCLE INFINIE ♾️          │
├─────────────────────────────────────────┤
│                                         │
│  loop {                                 │
│    ┌─────────────┐                      │
│    │ Exécuter    │                      │
│    │ code        │                      │
│    └─────────────┘                      │
│         │                                │
│         └─> Retour au début              │
│                                         │
│  break → Sortir! ✅                     │
│  continue → Passer au suivant! ✅      │
│                                         │
│  Infini jusqu'à break! ♾️              │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Boucle Infinie" - loop répète pour toujours jusqu'à ce que vous disiez break, comme une boucle infinie!

## Code Examples

### Example 1: Boucle Infinie

```rust
fn main() {
    let mut compteur = 0;
    
    loop {
        println!("Compteur: {}", compteur);
        compteur += 1;
        
        if compteur >= 5 {
            break;  // Sortir de la boucle
        }
    }
}
```

### Example 2: Retour de Valeur

```rust
fn main() {
    let mut compteur = 0;
    
    let resultat = loop {
        compteur += 1;
        
        if compteur == 10 {
            break compteur * 2;  // Retourne une valeur
        }
    };
    
    println!("Résultat: {}", resultat);  // 20
}
```

### Example 3: Labels pour Boucles Imbriquées

```rust
fn main() {
    let mut compteur = 0;
    
    'outer: loop {
        'inner: loop {
            compteur += 1;
            
            if compteur == 5 {
                break 'outer;  // Sortir de la boucle externe
            }
            
            if compteur == 3 {
                break 'inner;  // Sortir de la boucle interne
            }
        }
    }
    
    println!("Compteur final: {}", compteur);  // 5
}
```

## Official Resources

- [@official Rust Book - loop](https://doc.rust-lang.org/book/ch03-05-control-flow.html#repeating-code-with-loop)

