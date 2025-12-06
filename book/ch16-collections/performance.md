# Performance des Collections

## Learning Objectives

- Comprendre les caractéristiques de performance
- Choisir la bonne collection
- Optimiser les opérations

## Comparaisons

```
Vec<T>
├── Accès index: O(1)
├── Push: O(1) amorti
└── Insert: O(n)

HashMap<K, V>
├── Insert: O(1) amorti
├── Lookup: O(1) amorti
└── Hash collision: O(n) worst case
```

## Official Resources

- [@official Rust Book - Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)

