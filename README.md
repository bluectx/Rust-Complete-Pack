# 🦀 Le Guide Complet pour Apprendre Rust depuis le Néant totale !

<div align="center">

[![Dernière version](https://img.shields.io/github/v/tag/bluectx/Rust-Complete-Pack?label=version&sort=semver&style=flat-square&color=orange)](https://github.com/bluectx/Rust-Complete-Pack/tags)
[![GitHub stars](https://img.shields.io/github/stars/bluectx/Rust-Complete-Pack?style=flat-square&label=⭐%20stars&color=yellow)](https://github.com/bluectx/Rust-Complete-Pack)
[![GitHub forks](https://img.shields.io/github/forks/bluectx/Rust-Complete-Pack?style=flat-square&label=🍴%20forks&color=blue)](https://github.com/bluectx/Rust-Complete-Pack/network)
[![GitHub issues](https://img.shields.io/github/issues/bluectx/Rust-Complete-Pack?style=flat-square&label=issues&color=red)](https://github.com/bluectx/Rust-Complete-Pack/issues)
[![CI Status](https://img.shields.io/github/actions/workflow/status/bluectx/Rust-Complete-Pack/ci.yml?style=flat-square&label=CI)](https://github.com/bluectx/Rust-Complete-Pack/actions)
[![Licence](https://img.shields.io/github/license/bluectx/Rust-Complete-Pack?style=flat-square&label=licence)](LICENSE)
[![Profile views](https://komarev.com/ghpvc/?username=bluectx&label=Vues%20du%20profil&color=0e75b6&style=flat-square)](https://github.com/bluectx)

**Version :** 1.2.0  
**Dernière mise à jour :** 7 Décembre 2025  
**Public cible :** Débutant absolu → Expert

### Nouveautés Version 1.2.0

- ✅ **403 exercices complets** avec code de départ et tests
- ✅ Tous les exercices enrichis avec du code fonctionnel
- ✅ Progression structurée : Beginner (100) -> Intermediate (70) -> Advanced (34)
- ✅ Chaque exercice inclut CHALLENGE.md, src/main.rs et Cargo.toml. Amusez Vous!


</div>

---

##  Présentation du Projet

Bienvenue dans le **guide d'apprentissage Rust le plus complet** jamais créé, et en francais !

Ce dépôt est conçu pour accompagner **n'importe qui** même sans expérience de programmation depuis les bases absolues jusqu'à l'expertise avancée en Rust.

###  Pourquoi ce guide ?

Rust est un langage révolutionnaire créé par Mozilla pour résoudre un problème fondamental en programmation système :

- **Rapide** : compile en code machine ultra-performant (comme C/C++)
- **Sûr** : empêche les bugs mémoire (use-after-free, double-free) **avant** la compilation
- **Concurrent** : gère la parallélisation sans data races
- **Idiomatique** : encourage les bonnes pratiques par design

**Utilisé par :** Discord, Cloudflare, Figma, Meta, Amazon, Microsoft, Linux Kernel 6.1+, et bien d'autres.

###  Contenu du Guide

- ✅ **50 chapitres progressifs** (niveau 0 → expert)
- ✅ **403 exercices complets** avec code de départ et tests automatisés
- ✅ **6 mini-projets complets** et documentés
- ✅ **Cheatsheets imprimables** (ownership, lifetimes, traits, async)
- ✅ **Diagrammes et mindmaps** pour concepts complexes
- ✅ **Focus sécurité** : OPSEC, cryptographie, pentesting
- ✅ **Code exécutable** : tous les exemples fonctionnent

---

##  Table des Matières

1. [Présentation du projet](#-présentation-du-projet)
2. [Installation rapide](#-installation-rapide)
3. [Parcours d'apprentissage](#-parcours-dapprentissage)
4. [Table des matières détaillée](#-table-des-matières-détaillée)
5. [Objectifs pédagogiques](#-objectifs-pédagogiques-par-chapitre)
6. [Structure du dépôt](#-structure-du-dépôt)
7. [Accès direct aux fichiers](#-accès-direct-aux-fichiers)
8. [Mini-projets](#-mini-projets-pratiques)
9. [Feuille de route](#-feuille-de-route-apprentissage-10-semaines)
10. [Contribution](#-contribution)
11. [Licence](#-licence)
12. [Support et communauté](#-support--communauté)

---

## Installation | AnonSecLab

### Prérequis

Aucune connaissance préalable en programmation n'est requise ! 🎉

### Étape 1 : Installer Rust

**Linux / macOS :**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows :**
Télécharge et exécute [rustup-init.exe](https://rustup.rs/)

### Étape 2 : Vérifier l'installation

```bash
rustc --version
cargo --version
```

### Étape 3 : Cloner ce dépôt

```bash
git clone https://github.com/bluectx/Rust-Complete-Pack.git
cd Rust-Complete-Pack
```

### Étape 4 : Commencer l'apprentissage

```bash
cd book/ch01-foundations
cat 01-01-what-is-programming.md
```

---

## 🎓 Parcours d'Apprentissage

Le guide est structuré en **5 niveaux progressifs** :

| Niveau | Chapitres | Durée estimée | Compétences acquises |
|--------|-----------|---------------|---------------------|
| **🌱 Niveau 0 : Fondations** | 1-5 | 1 semaine | Comprendre la programmation, installer Rust, écrire premier programme |
| **🟢 Débutant** | 6-11 | 2 semaines | Variables, types, contrôle de flux, fonctions |
| **🟡 Intermédiaire** | 12-30 | 4 semaines | Ownership, lifetimes, traits, génériques, closures, modules |
| **🟠 Avancé** | 31-40 | 2 semaines | Collections, smart pointers, concurrence, async/await, macros |
| **🔴 Expert** | 41-50 | 1 semaine | Unsafe, FFI, WASM, embedded, sécurité, patterns avancés |

**Durée totale estimée :** 10 semaines à raison de 2-3h par jour.

---

##  Table des Matières Détaillée

### **🌱 NIVEAU 0 : FONDATIONS ABSOLUES (Chapitres 1-5)**

#### Chapitre 1 : Qu'est-ce qu'un Langage de Programmation ?
- 1.1 Ordinateurs et instructions
- 1.2 Qu'est-ce qu'un langage ?
- 1.3 Compilation vs interprétation
- 1.4 Pourquoi Rust ?
- 1.5 Ressources officielles

#### Chapitre 2 : Installation & Premier Programme
- 2.1 Installer rustup (Linux, macOS, Windows)
- 2.2 Vérifier l'installation
- 2.3 Éditions Rust (2021, 2024)
- 2.4 Créer premier projet avec cargo
- 2.5 Comprendre Cargo.toml

#### Chapitre 3 : Anatomie d'un Programme Rust
- 3.1 La fonction main()
- 3.2 Macro println!
- 3.3 Structure d'un fichier source
- 3.4 Commentaires
- 3.5 Exécution et debugging

#### Chapitre 4 : Variables et Mutabilité
- 4.1 Variables = boîtes pour stocker données
- 4.2 Immuabilité par défaut (mindset Rust)
- 4.3 Mot-clé `mut` pour mutabilité explicite
- 4.4 Scope de variable
- 4.5 Shadowing et réassignation

#### Chapitre 5 : Types Primitifs Fondamentaux
- 5.1 Entiers signés : i8, i16, i32, i64, i128, isize
- 5.2 Entiers non-signés : u8, u16, u32, u64, u128, usize
- 5.3 Débordement et wrap-around
- 5.4 Nombres flottants : f32, f64 et précision
- 5.5 Booléens (bool) et opérateurs logiques
- 5.6 Caractères (char) et Unicode

---

### **🟢 NIVEAU DÉBUTANT : CONCEPTS PRIMAIRES (Chapitres 6-11)**

#### Chapitre 6 : Tuples, Arrays et Slices
- 6.1 Tuples : collections hétérogènes
- 6.2 Déballage de tuples (destructuring)
- 6.3 Arrays : collections homogènes de taille fixe
- 6.4 Slices : vues sur arrays
- 6.5 Indexation et bounds checking

#### Chapitre 7 : String & &str - Les Deux Types de Chaînes
- 7.1 String : chaîne possédée, mutables, heap-allocated
- 7.2 &str : slice de string, immuable, littéraux
- 7.3 Pourquoi deux types ?
- 7.4 Conversion entre String et &str
- 7.5 Manipulation : chars, bytes, graphèmes

#### Chapitre 8 : Constantes vs Variables
- 8.1 Constantes : valeurs inchangeables à compile-time
- 8.2 Variables : état mutable à runtime
- 8.3 Shadowing : relier un nom à nouvelle valeur

#### Chapitre 9 : Conditions et Expressions
- 9.1 if/else if/else comme expressions
- 9.2 Opérateurs de comparaison
- 9.3 Opérateurs logiques (&&, ||, !)
- 9.4 Expressions ternaires implicites
- 9.5 Blocs et valeurs de retour

#### Chapitre 10 : Boucles
- 10.1 Boucle infinie : `loop { }`
- 10.2 Boucle conditionnelle : `while`
- 10.3 Boucle sur collection : `for`
- 10.4 Étiquettes de boucle et break
- 10.5 Continue et contrôle de flux

#### Chapitre 11 : Fonctions
- 11.1 Définition et paramètres
- 11.2 Types de paramètres
- 11.3 Valeurs de retour explicites
- 11.4 Expressions implicites (dernière ligne)
- 11.5 Scope et visibilité (pub)

---

### **🟡 NIVEAU INTERMÉDIAIRE : PROPRIÉTÉ & EMPRUNT (Chapitres 12-18) ⚠️ CRITICAL**

#### Chapitre 12 : Ownership - Les Trois Règles Fondamentales
- 12.1 Règle 1 : Chaque valeur a UN propriétaire
- 12.2 Règle 2 : Transfert (move) de propriété
- 12.3 Règle 3 : Suppression quand propriétaire drop
- 12.4 Visualiser ownership sur stack/heap
- 12.5 Move semantics vs copy semantics

#### Chapitre 13 : Copy Trait pour Types Simples
- 13.1 Quels types implémentent Copy ?
- 13.2 Copy = duplication automatique
- 13.3 Différence entre Copy et Move
- 13.4 Clone trait pour copie explicite

#### Chapitre 14 : Borrowing Immuable (&T)
- 14.1 Emprunter = lecture sans transfert
- 14.2 Syntaxe &variable
- 14.3 Immuabilité garantie
- 14.4 Multiples emprunts simultanés (OK)
- 14.5 Emprunts et scope

#### Chapitre 15 : Borrowing Mutable (&mut T)
- 15.1 Emprunt mutable = modification sans transfert
- 15.2 Syntaxe &mut variable
- 15.3 Règle XOR : UNE SEULE référence mutable
- 15.4 Pas de data races
- 15.5 Emprunt mutable vs immuable

#### Chapitre 16 : Règles du Borrow Checker
- 16.1 Le compilateur vérifie propriété + emprunts
- 16.2 Erreur : cannot borrow as mutable
- 16.3 Erreur : cannot move (borrowed value)
- 16.4 Dépannage système
- 16.5 Visualisation timeline emprunt

#### Chapitre 17 : Durées de Vie (Lifetimes)
- 17.1 Qu'est-ce qu'une lifetime ?
- 17.2 Annotation explicite : 'a, 'b, 'static
- 17.3 Lifetimes sur fonctions
- 17.4 Lifetimes sur structs
- 17.5 Élision implicite
- 17.6 Dangling references prevention

#### Chapitre 18 : Ownership Avancé
- 18.1 Patterns de possession
- 18.2 Retourner ownership
- 18.3 Tuple destructuring propriété
- 18.4 Scope imbriqué

---

### **🟡 NIVEAU INTERMÉDIAIRE : TYPES COMPOSÉS (Chapitres 19-25)**

#### Chapitre 19 : Structs - Regroupement Nommé
- 19.1 Définition de struct
- 19.2 Instantiation
- 19.3 Accès aux champs (field access)
- 19.4 Mutabilité au niveau struct
- 19.5 Struct patterns

#### Chapitre 20 : Implémentations et Méthodes (impl)
- 20.1 Blocs impl
- 20.2 Méthodes : self, &self, &mut self
- 20.3 Associated functions
- 20.4 Multiples blocs impl
- 20.5 Dérivation automatique (derive)

#### Chapitre 21 : Enums - Types Somme
- 21.1 Variantes enum
- 21.2 Enum avec données associées
- 21.3 Option\<T\> : absence vs présence
- 21.4 Result\<T, E\> : succès vs erreur
- 21.5 Enum methods

#### Chapitre 22 : Pattern Matching
- 22.1 Match expressions exhaustives
- 22.2 Patterns immuables
- 22.3 Destructuring dans patterns
- 22.4 Wildcard (_) et catch-all
- 22.5 Match guards (if conditions)

#### Chapitre 23 : Option\<T\> Profond
- 23.1 Absence de valeur sans null
- 23.2 Déballage : unwrap, expect
- 23.3 Chainable methods : map, and_then
- 23.4 if let et while let
- 23.5 Patterns sur Option

#### Chapitre 24 : Result\<T, E\> Profond
- 24.1 Succès (Ok) vs erreur (Err)
- 24.2 Propagation d'erreurs (? operator)
- 24.3 Chainable methods : map_err, or_else
- 24.4 Custom error types
- 24.5 Converting entre error types

#### Chapitre 25 : Traits - Interfaces et Contrats
- 25.1 Définition de trait
- 25.2 Implémentation de trait
- 25.3 Trait bounds sur generics
- 25.4 Multiple trait bounds
- 25.5 Traits standards : Display, Debug, Clone, Copy, Default

---

### **🟡 NIVEAU INTERMÉDIAIRE → AVANCÉ : GÉNÉRIQUES & ABSTRACTION (Chapitres 26-30)**

#### Chapitre 26 : Génériques - Paramètres de Type
- 26.1 Type parameters T
- 26.2 Génériques sur fonctions
- 26.3 Génériques sur structs
- 26.4 Génériques sur enums
- 26.5 Monomorphization

#### Chapitre 27 : Trait Bounds et Constraints
- 27.1 Single trait bound : T: Display
- 27.2 Multiple bounds : T: Display + Clone
- 27.3 Where clauses
- 27.4 Associated types
- 27.5 Higher-ranked trait bounds (HRTB)

#### Chapitre 28 : Closures - Fonctions Anonymes
- 28.1 Syntaxe : |x| x + 1
- 28.2 Type inference
- 28.3 Fn, FnMut, FnOnce traits
- 28.4 Capture d'environnement
- 28.5 Move keyword et ownership

#### Chapitre 29 : Itérateurs et Adaptateurs
- 29.1 Iterator trait : next() method
- 29.2 Adaptateurs : map, filter, fold
- 29.3 Consommateurs : collect, sum, count
- 29.4 Chaîner des adaptateurs
- 29.5 Lazy evaluation

#### Chapitre 30 : Modules et Organisation
- 30.1 Système de modules Rust
- 30.2 Chemin de module (path)
- 30.3 Visibilité (pub, pub(crate), pub(super))
- 30.4 Use statements et imports
- 30.5 Crate et package Cargo

---

### **🟠 NIVEAU AVANCÉ : CONCEPTS CRITIQUES (Chapitres 31-40)**

#### Chapitre 31 : Collections - Vec, HashMap, HashSet
- 31.1 Vec\<T\> : vecteurs dynamiques
- 31.2 VecDeque, LinkedList
- 31.3 HashMap : clé-valeur
- 31.4 BTreeMap : ordre trié
- 31.5 HashSet et BTreeSet
- 31.6 Capacité et réallocation

#### Chapitre 32 : Gestion d'Erreurs Explicite
- 32.1 Panics vs Recoverable errors
- 32.2 Result\<T, E\> comme pattern
- 32.3 Custom error types avec traits
- 32.4 Chaîner Results
- 32.5 Best practices OPSEC

#### Chapitre 33 : Smart Pointers - Box\<T\>
- 33.1 Box : allocation sur heap
- 33.2 Deref trait et coercion
- 33.3 Cas d'usage
- 33.4 Drop trait et RAII
- 33.5 Recursive types avec Box

#### Chapitre 34 : Smart Pointers - Rc\<T\> & RefCell\<T\>
- 34.1 Rc : reference counting
- 34.2 Multiple ownership
- 34.3 RefCell : interior mutability
- 34.4 Weak\<T\> : prévenir cycles
- 34.5 Debugging reference cycles

#### Chapitre 35 : Smart Pointers - Arc\<T\> & Mutex\<T\>
- 35.1 Arc : atomic reference counting (thread-safe)
- 35.2 Mutex : synchronisation
- 35.3 RwLock : reader-writer lock
- 35.4 Deadlock prevention
- 35.5 Condvar : condition variables

#### Chapitre 36 : Concurrence - Threads
- 36.1 Spawn de threads
- 36.2 Join et attendre résultat
- 36.3 Move closures et ownership
- 36.4 Send et Sync traits
- 36.5 Data races prevention

#### Chapitre 37 : Concurrence - Channels
- 37.1 Message passing : mpsc channels
- 37.2 Send et receive
- 37.3 Multiple producers
- 37.4 Synchronous vs asynchronous
- 37.5 Blocking channels

#### Chapitre 38 : Async/Await Basics
- 38.1 Async functions
- 38.2 Await operator
- 38.3 Futures trait
- 38.4 Async blocks
- 38.5 Runtime requirements

#### Chapitre 39 : Tokio Runtime et Networking
- 39.1 Tokio : async runtime populaire
- 39.2 Tokio::spawn pour tasks
- 39.3 TCP sockets
- 39.4 HTTP client/server basics
- 39.5 Timeouts et cancellation

#### Chapitre 40 : Macros Déclaratives
- 40.1 Macro rules! syntax
- 40.2 Patterns de macro
- 40.3 Repetition ($(...)*,)
- 40.4 Debugging macros
- 40.5 Common macros (println!, vec!, etc.)

---

### **🔴 NIVEAU EXPERT : CONCEPTS AVANCÉS (Chapitres 41-50)**

#### Chapitre 41 : Macros Procédurales
- 41.1 Derive macros
- 41.2 Attribute macros
- 41.3 Function-like macros
- 41.4 TokenStream manipulation
- 41.5 Debugging procedural macros

#### Chapitre 42 : Unsafe & Raw Pointers
- 42.1 Quand utiliser unsafe
- 42.2 Dereferencing raw pointers (*const, *mut)
- 42.3 Unsafe functions
- 42.4 Mutable statics
- 42.5 Unions et packed structs

#### Chapitre 43 : FFI - Foreign Function Interface
- 43.1 Binding C libraries
- 43.2 Type correspondences (C → Rust)
- 43.3 extern "C" functions
- 43.4 Callbacks depuis C
- 43.5 Memory safety concerns

#### Chapitre 44 : Memory Model & Layout
- 44.1 Stack vs Heap
- 44.2 Alignment et padding
- 44.3 Size, size_of_val
- 44.4 Zero-copy abstractions
- 44.5 Escape analysis

#### Chapitre 45 : Compiler Internals
- 45.1 Compilation phases
- 45.2 MIR (Mid-level IR)
- 45.3 LLVM backend
- 45.4 Debug vs Release
- 45.5 Link-time optimization (LTO)

#### Chapitre 46 : Testing & Benchmarking
- 46.1 Unit tests (#[test])
- 46.2 Integration tests
- 46.3 Doc tests
- 46.4 Criterion benchmarking
- 46.5 Fuzzing et property testing

#### Chapitre 47 : Tooling & Cargo Avancé
- 47.1 cargo-expand (voir MIR)
- 47.2 cargo-clippy (linting)
- 47.3 cargo-audit (vulnerabilités)
- 47.4 cargo-tree (dépendances)
- 47.5 Build scripts (build.rs)

#### Chapitre 48 : WASM & Embedded
- 48.1 Compilation vers WebAssembly
- 48.2 wasm-bindgen
- 48.3 Performance WASM
- 48.4 no_std programming
- 48.5 Embedded Rust basics

#### Chapitre 49 : Sécurité & OPSEC
- 49.1 Buffer overflow prevention
- 49.2 Integer overflow/underflow
- 49.3 Use-after-free prevention
- 49.4 Constant-time comparisons
- 49.5 Secret management (zeroize)

#### Chapitre 50 : Patterns & Best Practices
- 50.1 Builder pattern
- 50.2 Type state pattern
- 50.3 RAII (Resource Acquisition Is Initialization)
- 50.4 Newtype pattern
- 50.5 Composing abstractions

---

##  Objectifs Pédagogiques par Chapitre

### Chapitres 1-3 : Fondations

**Après ces chapitres, vous comprendrez :**
- ✅ Qu'un langage de programmation = grammaire pour parler à un ordinateur
- ✅ Comment installer Rust et créer un premier projet
- ✅ Anatomie d'un programme simple
- ✅ Comment exécuter et déboguer du code

**Vocabulaire clé :**
- Compilateur, bytecode, machine code, assembly
- Source code, executable, binary
- REPL vs compiled languages

---

### Chapitres 4-8 : Types & Variables

**Après ces chapitres, vous pourrez :**
- ✅ Déclarer variables immuables et mutables
- ✅ Utiliser tous les types primitifs correctement
- ✅ Comprendre collections (tuples, arrays, slices)
- ✅ Manipuler chaînes (String vs &str)

**Vocabulaire clé :**
- Variable, assignation, mutation, shadowing
- Type inference, casting, type coercion
- Stack frame, scope, lifetime
- Owned vs borrowed

---

### Chapitres 9-11 : Contrôle de Flux & Fonctions

**Après ces chapitres, vous pourrez :**
- ✅ Brancher le code avec conditions
- ✅ Boucler efficacement avec for/while/loop
- ✅ Décomposer code en fonctions réutilisables
- ✅ Passer arguments et retourner valeurs

**Vocabulaire clé :**
- Expression vs statement
- Guard, match, exhaustiveness
- Function signature, return type

---

### Chapitres 12-18 : Ownership & Lifetimes ⚠️ **LE CŒUR DE RUST**

**Après ces chapitres, vous comprendrez :**
- ✅ Pourquoi Rust est sûr sans garbage collector
- ✅ Comment le transfert (move) de propriété fonctionne
- ✅ Emprunter sans transférer (borrowing)
- ✅ Éviter dangling references avec lifetimes
- ✅ Design d'API safe et efficace

**Vocabulaire clé :**
- Ownership, move, copy, clone
- Borrow, reference, mutable borrow
- Lifetime, scope, drop
- Borrow checker, data race

**Compétences acquises :**
- Corriger TOUS les erreurs "cannot borrow"
- Écrire du code safe sans null pointers
- Profiler et optimiser allocations

---

### Chapitres 19-25 : Types Composés & Traits

**Après ces chapitres, vous pourrez :**
- ✅ Modéliser un domaine avec structs/enums
- ✅ Implémenter méthodes et behaviors
- ✅ Utiliser traits pour abstraction
- ✅ Écrire du code réutilisable avec generics

**Vocabulaire clé :**
- Struct, field, method
- Enum, variant, pattern
- Trait, implementation, bound
- Generic type parameter

---

### Chapitres 26-30 : Programmation Fonctionnelle

**Après ces chapitres, vous pourrez :**
- ✅ Utiliser closures et itérateurs
- ✅ Chaîner transformations fonctionnelles
- ✅ Organiser code en modules/crates
- ✅ Comprendre lazy evaluation

**Vocabulaire clé :**
- Closure, capture, move
- Iterator, adapter, consumer
- Module, crate, path
- Visibility, pub

---

### Chapitres 31-40 : Concurrence & Async

**Après ces chapitres, vous pourrez :**
- ✅ Écrire du code concurrent sans data races
- ✅ Utiliser threads et message passing
- ✅ Écrire du code async/await avec tokio
- ✅ Implémenter serveurs et clients réseau

**Vocabulaire clé :**
- Thread, task, future
- Channel, mutex, rwlock
- Async, await, runtime
- Send, sync

---

### Chapitres 41-50 : Concepts Avancés

**Après ces chapitres, vous serez :**
- ✅ Capable d'utiliser unsafe code consciemment
- ✅ Capable d'interfacer C libraries (FFI)
- ✅ Capable de compiler vers WASM
- ✅ Capable d'écrire du code embedded
- ✅ Expert en sécurité & OPSEC Rust

**Vocabulaire clé :**
- Unsafe, raw pointer, transmute
- FFI, extern "C", ABI
- WASM, no_std, bare-metal
- Fuzzing, sanitizer, MIRI

---

## 📂 Structure du Dépôt

```
Rust-Complete-Pack/
│
├── README.md                    # Ce fichier
├── SUMMARY.md                   # Vue d'ensemble de la structure
├── Cargo.toml                   # Workspace root
├── .cursorrules                 # Règles de génération
│
├── book/                              # Contenu pédagogique principal
│   ├── SUMMARY.md
│   ├── ch01-foundations/                    # Chapitre 1
│   │   ├── 01-01-what-is-programming.md
│   │   ├── 01-02-what-is-rust.md
│   │   ├── 01-03-installation.md
│   │   ├── 01-04-hello-world.md
│   │   └── 01-05-anatomy.md
│   ├── ch02-primitives/         # Chapitre 2
│   ├── ch03-composites/         # Chapitre 3
│   ├── ...                      # Chapitres 4-49
│   └── ch50-patterns/           # Chapitre 50
│
├── exercises/                   #  Exercices pratiques
│   ├── ch01-exercises/
│   │   ├── ex01-hello-world/
│   │   │   ├── Cargo.toml
│   │   │   ├── src/main.rs
│   │   │   ├── tests/solution_tests.rs
│   │   │   ├── CHALLENGE.md
│   │   │   └── SOLUTION.md
│   │   ├── ex02-variables/
│   │   └── ex03-types/
│   ├── ch02-exercises/
│   ├── ...
│   └── EXERCISE_INDEX.md
│
├── projects/                    #  Mini-projets complets
│   ├── p01-hello-world/
│   │   ├── Cargo.toml
│   │   ├── src/main.rs
│   │   ├── tests/integration_tests.rs
│   │   └── README.md
│   ├── p02-cli-calculator/
│   ├── p03-mini-http-server/
│   ├── p04-data-pipeline/
│   ├── p05-terminal-game/
│   ├── p06-web-service/
│   └── PROJECTS_INDEX.md
│
├── cheatsheets/                       #  Aide-mémoire imprimables
│   ├── ownership-lifetimes.md
│   ├── pattern-matching.md
│   ├── traits-bounds.md
│   ├── closures.md
│   ├── async-await.md
│   └── security-checklist.md
│
├── assets/                      #  Ressources visuelles
    ├── mindmaps/
    ├── diagrams/
    │   ├── ownership-timeline.txt
    │   ├── borrow-rules.txt
    │   └── memory-layout.txt
    └── printables/
        └── cheatsheet-templates.pdf


```

---

## Accès Direct aux Fichiers

### Chapitres du Guide (`book/`)

#### Niveau 0 : Fondations (Chapitres 1-5)

- **[Chapitre 01 : Foundations](book/ch01-foundations/)**
  - [01-01 What Is Programming](book/ch01-foundations/01-01-what-is-programming.md)
  - [01-02 What Is Rust](book/ch01-foundations/01-02-what-is-rust.md)
  - [01-03 Installation](book/ch01-foundations/01-03-installation.md)
  - [01-04 Hello World](book/ch01-foundations/01-04-hello-world.md)
  - [01-05 Anatomy](book/ch01-foundations/01-05-anatomy.md)

- **[Chapitre 02 : Primitives](book/ch02-primitives/)**
  - [Bool Char](book/ch02-primitives/bool-char.md)
  - [Constants](book/ch02-primitives/constants.md)
  - [Numeric Types](book/ch02-primitives/numeric-types.md)
  - [Type Inference](book/ch02-primitives/type-inference.md)
  - [Variables](book/ch02-primitives/variables.md)

- **[Chapitre 03 : Composites](book/ch03-composites/)**
  - [Arrays](book/ch03-composites/arrays.md)
  - [Slices](book/ch03-composites/slices.md)
  - [String Manipulation](book/ch03-composites/string-manipulation.md)
  - [Strings](book/ch03-composites/strings.md)
  - [Tuples](book/ch03-composites/tuples.md)

- **[Chapitre 04 : Control Flow](book/ch04-control-flow/)**
  - [For](book/ch04-control-flow/for.md)
  - [If Else](book/ch04-control-flow/if-else.md)
  - [Loop](book/ch04-control-flow/loop.md)
  - [Pattern Matching](book/ch04-control-flow/pattern-matching.md)
  - [While](book/ch04-control-flow/while.md)

- **[Chapitre 05 : Functions](book/ch05-functions/)**
  - [Declaration](book/ch05-functions/declaration.md)
  - [Doc Comments](book/ch05-functions/doc-comments.md)
  - [Implicit Return](book/ch05-functions/implicit-return.md)
  - [Parameters](book/ch05-functions/parameters.md)
  - [Scope](book/ch05-functions/scope.md)

#### Niveau Débutant (Chapitres 6-11)

- **[Chapitre 06 : Ownership](book/ch06-ownership/)**
  - [Borrowing](book/ch06-ownership/borrowing.md)
  - [Ownership Rules](book/ch06-ownership/ownership-rules.md)

- **[Chapitre 07 : Lifetimes](book/ch07-lifetimes/)**
  - [Annotations](book/ch07-lifetimes/annotations.md)
  - [Elision](book/ch07-lifetimes/elision.md)
  - [Memory Diagrams](book/ch07-lifetimes/memory-diagrams.md)
  - [Parameters](book/ch07-lifetimes/parameters.md)
  - [Static Lifetime](book/ch07-lifetimes/static-lifetime.md)

- **[Chapitre 08 : Structs](book/ch08-structs/)**
  - [Associated Functions](book/ch08-structs/associated-functions.md)
  - [Definition](book/ch08-structs/definition.md)
  - [Instantiation](book/ch08-structs/instantiation.md)
  - [Methods](book/ch08-structs/methods.md)
  - [Move Copy](book/ch08-structs/move-copy.md)
  - [Tuple Structs](book/ch08-structs/tuple-structs.md)

- **[Chapitre 09 : Enums](book/ch09-enums/)**
  - [Comparisons](book/ch09-enums/comparisons.md)
  - [Definition](book/ch09-enums/definition.md)
  - [Enums With Data](book/ch09-enums/enums-with-data.md)
  - [Option Result](book/ch09-enums/option-result.md)
  - [Pattern Matching](book/ch09-enums/pattern-matching.md)

- **[Chapitre 10 : Traits](book/ch10-traits/)**
  - [Bounds](book/ch10-traits/bounds.md)
  - [Definition](book/ch10-traits/definition.md)
  - [Implementation](book/ch10-traits/implementation.md)
  - [Trait Objects](book/ch10-traits/trait-objects.md)
  - [Where Clauses](book/ch10-traits/where-clauses.md)

- **[Chapitre 11 : Generics](book/ch11-generics/)**
  - [Associated Types](book/ch11-generics/associated-types.md)
  - [Const Generics](book/ch11-generics/const-generics.md)
  - [Functions](book/ch11-generics/functions.md)
  - [HRTBs](book/ch11-generics/hrtbs.md)
  - [Structs Enums](book/ch11-generics/structs-enums.md)

#### Niveau Intermédiaire (Chapitres 12-30)

- **[Chapitre 12 : Closures](book/ch12-closures/)**
  - [Captures](book/ch12-closures/captures.md)
  - [Fn Traits](book/ch12-closures/fn-traits.md)
  - [Idioms](book/ch12-closures/idioms.md)
  - [Move](book/ch12-closures/move.md)
  - [Syntax](book/ch12-closures/syntax.md)

- **[Chapitre 13 : Iterators](book/ch13-iterators/)**
  - [Adapters](book/ch13-iterators/adapters.md)
  - [Consumers](book/ch13-iterators/consumers.md)
  - [Creation](book/ch13-iterators/creation.md)
  - [Lazy Evaluation](book/ch13-iterators/lazy-evaluation.md)

- **[Chapitre 14 : Errors](book/ch14-errors/)**
  - [Anyhow](book/ch14-errors/anyhow.md)
  - [Propagation](book/ch14-errors/propagation.md)
  - [Result Option](book/ch14-errors/result-option.md)
  - [Thiserror](book/ch14-errors/thiserror.md)
  - [Unwrap Expect](book/ch14-errors/unwrap-expect.md)

- **[Chapitre 15 : Modules](book/ch15-modules/)**
  - [Crate Root](book/ch15-modules/crate-root.md)
  - [Modules](book/ch15-modules/modules.md)
  - [Use Paths](book/ch15-modules/use-paths.md)
  - [Visibility](book/ch15-modules/visibility.md)
  - [Workspaces](book/ch15-modules/workspaces.md)

- **[Chapitre 16 : Collections](book/ch16-collections/)**
  - [Hashmap](book/ch16-collections/hashmap.md)
  - [Slices](book/ch16-collections/slices.md)
  - [String](book/ch16-collections/string.md)
  - [Vec](book/ch16-collections/vec.md)

- **[Chapitre 17 : Smart Pointers](book/ch17-smart-pointers/)**
  - [Arc](book/ch17-smart-pointers/arc.md)
  - [Box](book/ch17-smart-pointers/box.md)
  - [Interior Mutability](book/ch17-smart-pointers/interior-mutability.md)
  - [Rc](book/ch17-smart-pointers/rc.md)
  - [RefCell](book/ch17-smart-pointers/refcell.md)

- **[Chapitre 18 : Concurrency](book/ch18-concurrency/)**
  - [Arc Mutex](book/ch18-concurrency/arc-mutex.md)
  - [Channels](book/ch18-concurrency/channels.md)
  - [RwLock](book/ch18-concurrency/rwlock.md)
  - [Send Sync](book/ch18-concurrency/send-sync.md)
  - [Threads](book/ch18-concurrency/threads.md)

- **[Chapitre 19 : Async Await](book/ch19-async-await/)**
  - [Async Await](book/ch19-async-await/async-await.md)
  - [Futures](book/ch19-async-await/futures.md)
  - [Introduction](book/ch19-async-await/introduction.md)
  - [Select Spawn](book/ch19-async-await/select-spawn.md)
  - [Tokio](book/ch19-async-await/tokio.md)

- **[Chapitre 20 : Macros](book/ch20-macros/)**
  - [Declarative](book/ch20-macros/declarative.md)
  - [Derive](book/ch20-macros/derive.md)
  - [Procedural](book/ch20-macros/procedural.md)
  - [When To Use](book/ch20-macros/when-to-use.md)

- **[Chapitre 21 : Unsafe](book/ch21-unsafe/)**
  - [Best Practices](book/ch21-unsafe/best-practices.md)
  - [FFI](book/ch21-unsafe/ffi.md)
  - [Raw Pointers](book/ch21-unsafe/raw-pointers.md)
  - [Unsafe Blocks](book/ch21-unsafe/unsafe-blocks.md)
  - [When To Use](book/ch21-unsafe/when-to-use.md)

- **[Chapitre 22 : Compiler](book/ch22-compiler/)**
  - [Introduction](book/ch22-compiler/introduction.md)

- **[Chapitre 23 : Testing](book/ch23-testing/)**
  - [Introduction](book/ch23-testing/introduction.md)

- **[Chapitre 24 : Cargo](book/ch24-cargo/)**
  - [Introduction](book/ch24-cargo/introduction.md)

- **[Chapitre 26 : Memory](book/ch26-memory/)**
  - [Introduction](book/ch26-memory/introduction.md)

- **[Chapitre 28 : WASM](book/ch28-wasm/)**
  - [Introduction](book/ch28-wasm/introduction.md)

- **[Chapitre 29 : Networking](book/ch29-networking/)**
  - [Introduction](book/ch29-networking/introduction.md)

- **[Chapitre 30 : OS Dev](book/ch30-os-dev/)**
  - [Introduction](book/ch30-os-dev/introduction.md)

#### Niveau Avancé (Chapitres 31-50)

> **Note :** Les chapitres 31-50 contiennent plusieurs sous-thèmes par numéro de chapitre. Consultez directement le dossier `book/` pour la liste complète.

---

## Mini-Projets Pratiques ! 
**( parce que je trouve sa plus ludique d'apprendre avec exercice plutot que de la theorie encore et encore )**

---


### Projet 1 : Hello World Avancé
**Niveau :** 🌱 Débutant  
**Concepts :** println!, variables, types  
📂 [Voir le projet](projects/p01-hello-world/)

### Projet 2 : Calculatrice CLI
**Niveau :** 🟢 Débutant  
**Concepts :** input/output, match, functions, error handling  
📂 [Voir le projet](projects/p02-cli-calculator/)

### Projet 3 : Mini HTTP Server
**Niveau :** 🟡 Intermédiaire  
**Concepts :** TCP sockets, threads, ownership  
📂 [Voir le projet](projects/p03-mini-http-server/)

### Projet 4 : Data Pipeline
**Niveau :** 🟡 Intermédiaire  
**Concepts :** iterators, closures, file I/O  
📂 [Voir le projet](projects/p04-data-pipeline/)

### Projet 5 : Terminal Game
**Niveau :** 🟠 Avancé  
**Concepts :** async/await, event loop, state machine  
📂 [Voir le projet](projects/p05-terminal-game/)

### Projet 6 : Web Service
**Niveau :** 🔴 Expert  
**Concepts :** Tokio, databases, authentication  
📂 [Voir le projet](projects/p06-web-service/)

---

## Roadmap (10 Semaines)

| Semaine | Chapitres | Focus | Projet |
|---------|-----------|-------|--------|
| **1** | 1-5 | Installation, bases absolues | Hello World Avancé |
| **2** | 6-11 | Variables, types, contrôle de flux | Calculatrice CLI |
| **3-4** | 12-18 | Ownership, lifetimes, borrowing | - |
| **5** | 19-25 | Structs, enums, traits | - |
| **6** | 26-30 | Génériques, closures, modules | Data Pipeline |
| **7** | 31-35 | Collections, smart pointers | - |
| **8** | 36-40 | Concurrence, async/await, macros | Mini HTTP Server |
| **9** | 41-46 | Unsafe, FFI, testing | - |
| **10** | 47-50 | WASM, embedded, patterns | Web Service |

**Temps estimé par jour :** 2-3 heures  
**Validation :** Compléter tous les exercices de chaque chapitre

---

## Contribution

Les contributions sont les bienvenues ! 🎉

### Comment contribuer

1. **Fork** le dépôt
2. Créez une **branche** : `git checkout -b feature/votre-fork`
3. **Commit** vos changements : `git commit -m 'ajout de votre fork'`
4. **Push** vers la branche : `git push origin feature/votre-fork`
5. Ouvrez une **Pull Request**

### Types de contributions

- Correction de fautes / typos
- Correction de bugs dans les exemples
- Ajout de nouveaux exercices
- Amélioration de la documentation
- Traductions (anglais, espagnol, etc.)
- Diagrammes et visualisations

### Guidelines

- Respectez le style de code Rust (`cargo fmt`)
- Ajoutez des tests pour les nouveaux exemples
- Documentez votre code avec des commentaires clairs
- Suivez la structure existante

---

## Licence

Ce projet est sous licence **MIT** - voir le fichier [LICENSE](LICENSE) pour plus de détails.

---

## Support & Communauté

### Besoin d'aide ?

- **Email** : contact@anonseclab.org
- **Discord** : [Rejoindre le serveur](https://discord.gg/ZCKjth8TGm)

### Ressources

- 📖 [The Rust Book (officiel)](https://doc.rust-lang.org/book/)
- 🦀 [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- 🏋️ [Rustlings](https://github.com/rust-lang/rustlings)
- 📺 [Let's Get Rusty (YouTube)](https://www.youtube.com/@letsgetrusty)
- 🗣️ [r/rust (Reddit)](https://www.reddit.com/r/rust/)

### Suivre le Projet

- ⭐ **Star** ce dépôt pour suivre les mises à jour
-    **Watch** pour recevoir des notifications
-    **Fork** pour créer votre version personnalisée

---

<div align="center">

**Créé avec ❤️ par AnonSecLab pour la communauté Rust**

[![GitHub](https://img.shields.io/badge/GitHub-bluectx-181717?style=for-the-badge&logo=github)](https://github.com/bluectx)
[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)

**[⬆ Retour en haut](#-le-guide-complet-pour-apprendre-rust-depuis-zéro)**

</div>
 