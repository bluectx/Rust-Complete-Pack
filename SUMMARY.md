# Rust Learning Guide - Table des Matières

## Niveau 0 : Fondations Absolues

### Chapitre 1 : Fondations
- [01-01: Qu'est-ce que la programmation?](book/ch01-foundations/01-01-what-is-programming.md)
- [01-02: Qu'est-ce que Rust?](book/ch01-foundations/01-02-what-is-rust.md)
- [01-03: Installation](book/ch01-foundations/01-03-installation.md)
- [01-04: Hello World](book/ch01-foundations/01-04-hello-world.md)
- [01-05: Anatomie d'un programme](book/ch01-foundations/01-05-anatomy.md)

### Chapitre 2 : Types Primitifs
- [Variables et mutabilité](book/ch02-primitives/variables.md)
- [Types entiers et flottants](book/ch02-primitives/numeric-types.md)
- [Booléens et caractères](book/ch02-primitives/bool-char.md)
- [Inférence de types](book/ch02-primitives/type-inference.md)
- [Constantes vs Variables](book/ch02-primitives/constants.md)

### Chapitre 3 : Types Composés
- [Tuples](book/ch03-composites/tuples.md)
- [Tableaux](book/ch03-composites/arrays.md)
- [Slices](book/ch03-composites/slices.md)
- [String vs &str](book/ch03-composites/strings.md)
- [Manipulation de chaînes](book/ch03-composites/string-manipulation.md)

### Chapitre 4 : Contrôle de Flux
- [Expressions if/else](book/ch04-control-flow/if-else.md)
- [Boucles loop](book/ch04-control-flow/loop.md)
- [Boucles while](book/ch04-control-flow/while.md)
- [Boucles for](book/ch04-control-flow/for.md)
- [Pattern matching intro](book/ch04-control-flow/pattern-matching.md)

### Chapitre 5 : Fonctions
- [Déclaration de fonctions](book/ch05-functions/declaration.md)
- [Paramètres et valeurs de retour](book/ch05-functions/parameters.md)
- [Retour implicite](book/ch05-functions/implicit-return.md)
- [Portée et stack frames](book/ch05-functions/scope.md)
- [Commentaires de documentation](book/ch05-functions/doc-comments.md)

## Niveau 1 : Concepts Fondamentaux Avancés

### Chapitre 6 : Ownership & Borrowing
- [Règles d'ownership](book/ch06-ownership/ownership-rules.md)
- [Move vs Copy](book/ch06-ownership/move-copy.md)
- [Borrowing](book/ch06-ownership/borrowing.md)
- [Règle XOR](book/ch06-ownership/xor-rule.md)
- [Comparaisons C/C++](book/ch06-ownership/comparisons.md)

### Chapitre 7 : Lifetimes
- [Annotations de lifetime](book/ch07-lifetimes/annotations.md)
- [Lifetime 'static](book/ch07-lifetimes/static-lifetime.md)
- [Elision de lifetimes](book/ch07-lifetimes/elision.md)
- [Lifetime parameters](book/ch07-lifetimes/parameters.md)
- [Diagrammes mémoire](book/ch07-lifetimes/memory-diagrams.md)

### Chapitre 8 : Structures
- [Définition de structs](book/ch08-structs/definition.md)
- [Instanciation](book/ch08-structs/instantiation.md)
- [Méthodes](book/ch08-structs/methods.md)
- [Associated functions](book/ch08-structs/associated-functions.md)
- [Tuple structs](book/ch08-structs/tuple-structs.md)

### Chapitre 9 : Enums
- [Définition d'enums](book/ch09-enums/definition.md)
- [Pattern matching avec enums](book/ch09-enums/pattern-matching.md)
- [Enums avec données](book/ch09-enums/enums-with-data.md)
- [Option et Result](book/ch09-enums/option-result.md)
- [Comparaisons avec autres langages](book/ch09-enums/comparisons.md)

### Chapitre 10 : Traits
- [Définition de traits](book/ch10-traits/definition.md)
- [Implémentation de traits](book/ch10-traits/implementation.md)
- [Trait bounds](book/ch10-traits/bounds.md)
- [Where clauses](book/ch10-traits/where-clauses.md)
- [Trait objects (dyn)](book/ch10-traits/trait-objects.md)

### Chapitre 11 : Generics
- [Fonctions génériques](book/ch11-generics/functions.md)
- [Structs et enums génériques](book/ch11-generics/structs-enums.md)
- [Const generics](book/ch11-generics/const-generics.md)
- [Associated types](book/ch11-generics/associated-types.md)
- [HRTBs (Higher-Ranked Trait Bounds)](book/ch11-generics/hrtbs.md)

### Chapitre 12 : Closures
- [Syntaxe des closures](book/ch12-closures/syntax.md)
- [Fn, FnMut, FnOnce](book/ch12-closures/fn-traits.md)
- [Mot-clé move](book/ch12-closures/move.md)
- [Captures](book/ch12-closures/captures.md)
- [Idiomes courants](book/ch12-closures/idioms.md)

### Chapitre 13 : Iterators
- [Création d'itérateurs](book/ch13-iterators/creation.md)
- [Adapters](book/ch13-iterators/adapters.md)
- [Consumers](book/ch13-iterators/consumers.md)
- [Évaluation paresseuse](book/ch13-iterators/lazy-evaluation.md)
- [Performance](book/ch13-iterators/performance.md)

### Chapitre 14 : Gestion d'Erreurs
- [Result et Option](book/ch14-errors/result-option.md)
- [unwrap et expect](book/ch14-errors/unwrap-expect.md)
- [Propagation d'erreurs](book/ch14-errors/propagation.md)
- [anyhow](book/ch14-errors/anyhow.md)
- [thiserror](book/ch14-errors/thiserror.md)

### Chapitre 15 : Modules & Crates
- [Système de modules](book/ch15-modules/modules.md)
- [Visibilité](book/ch15-modules/visibility.md)
- [use et paths](book/ch15-modules/use-paths.md)
- [Crate root](book/ch15-modules/crate-root.md)
- [Workspaces](book/ch15-modules/workspaces.md)

### Chapitre 16 : Collections
- [Vec](book/ch16-collections/vec.md)
- [HashMap](book/ch16-collections/hashmap.md)
- [String](book/ch16-collections/string.md)
- [Slices](book/ch16-collections/slices.md)
- [Caractéristiques de performance](book/ch16-collections/performance.md)

### Chapitre 17 : Smart Pointers
- [Box](book/ch17-smart-pointers/box.md)
- [Rc](book/ch17-smart-pointers/rc.md)
- [Arc](book/ch17-smart-pointers/arc.md)
- [RefCell](book/ch17-smart-pointers/refcell.md)
- [Interior mutability](book/ch17-smart-pointers/interior-mutability.md)

### Chapitre 18 : Concurrence
- [Threads](book/ch18-concurrency/threads.md)
- [Channels](book/ch18-concurrency/channels.md)
- [Arc et Mutex](book/ch18-concurrency/arc-mutex.md)
- [RwLock](book/ch18-concurrency/rwlock.md)
- [Send et Sync](book/ch18-concurrency/send-sync.md)

### Chapitre 19 : Async/Await
- [Introduction à async](book/ch19-async-await/introduction.md)
- [tokio](book/ch19-async-await/tokio.md)
- [Futures](book/ch19-async-await/futures.md)
- [async/await](book/ch19-async-await/async-await.md)
- [select! et spawn](book/ch19-async-await/select-spawn.md)

### Chapitre 20 : Macros
- [Macros déclaratives](book/ch20-macros/declarative.md)
- [Macros procédurales](book/ch20-macros/procedural.md)
- [derive macros](book/ch20-macros/derive.md)
- [Quand utiliser des macros](book/ch20-macros/when-to-use.md)

### Chapitre 21 : Unsafe Rust
- [Blocs unsafe](book/ch21-unsafe/unsafe-blocks.md)
- [Raw pointers](book/ch21-unsafe/raw-pointers.md)
- [FFI](book/ch21-unsafe/ffi.md)
- [Quand utiliser unsafe](book/ch21-unsafe/when-to-use.md)
- [Bonnes pratiques unsafe](book/ch21-unsafe/best-practices.md)

## Sections Spécialisées

### Performance & Profiling
- [cargo bench](book/performance/cargo-bench.md)
- [criterion.rs](book/performance/criterion.md)
- [flamegraph](book/performance/flamegraph.md)
- [miri](book/performance/miri.md)
- [Zero-cost abstractions](book/performance/zero-cost.md)

### Sécurité
- [Patterns unsafe](book/security/unsafe-patterns.md)
- [Vulnérabilités FFI](book/security/ffi-vulnerabilities.md)
- [Send et Sync](book/security/send-sync.md)
- [Limites du borrow checker](book/security/borrow-checker-limits.md)
- [Audit de crates](book/security/crate-audit.md)

### Patterns Rust Pro
- [Builder pattern](book/patterns/builder.md)
- [Newtype pattern](book/patterns/newtype.md)
- [Type-state pattern](book/patterns/type-state.md)
- [Phantom types](book/patterns/phantom-types.md)
- [Error propagation](book/patterns/error-propagation.md)
- [Ergonomie API](book/patterns/api-ergonomics.md)

## Ressources

- [Cheatsheets](cheatsheets/)
- [Exercices](exercises/)
- [Projets](projects/)

