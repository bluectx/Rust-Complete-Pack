# Quick Reference - Rust Essentials 🍔

## Collections

```
Vec<T>        → Liste ordonnée (comme étagère)
HashMap<K,V>  → Agenda clé→valeur (Lundi = Burger 🍔)
HashSet<T>    → Collection unique (pas de doublons)
```

## Smart Pointers

```
Box<T>    → Boîte magique (heap allocation)
Rc<T>     → Compteur de partage (single-threaded)
Arc<T>    → Compteur thread-safe (multi-threaded)
RefCell<T> → Interior mutability
```

## Error Handling

```
Result<T,E> → Ok(valeur) ou Err(erreur)
Option<T>    → Some(valeur) ou None
? operator   → Propagation d'erreurs
```

## Mnémonique

**"Vec = Étagère, HashMap = Agenda Burger, Result = Ok ou Pas Ok!"**

