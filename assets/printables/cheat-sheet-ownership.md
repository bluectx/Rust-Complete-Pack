# Cheat Sheet - Ownership & Borrowing 🍔

## Règles Rapides

### Les 3 Règles d'Ownership

```
1. UNE BURGER, UN PROPRIÉTAIRE 🍔
   - Chaque valeur a un propriétaire
   - Pas de partage d'ownership

2. MOVE = DONNER LA BURGER
   - let b = a;  // a est déplacé vers b
   - a n'est plus utilisable

3. BORROW = PRÊTER LA BURGER
   - let b = &a;  // b emprunte a
   - a reste utilisable
```

## Mnémonique

**"Une Burger, Un Propriétaire, Move = Donner, Borrow = Prêter!"**

## Syntaxe Rapide

```rust
// Ownership
let a = String::from("Burger 🍔");
let b = a;  // MOVE: a n'est plus valide

// Borrowing immuable
let a = String::from("Burger 🍔");
let b = &a;  // BORROW: a reste valide

// Borrowing mutable
let mut a = String::from("Burger 🍔");
let b = &mut a;  // BORROW mutable: une seule référence
```

## Erreurs Courantes

```
❌ cannot move out of borrowed value
✅ Utiliser clone() ou borrowing

❌ cannot borrow as mutable
✅ Vérifier qu'il n'y a pas d'autres emprunts
```

