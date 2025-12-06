# Le Compilateur Rust

## Learning Objectives

- Comprendre le processus de compilation Rust
- Connaître les phases de compilation
- Utiliser les options du compilateur
- Comprendre les messages d'erreur

## Key Vocabulary

| Term | Definition |
|------|-----------|
| rustc | Compilateur Rust |
| AST | Abstract Syntax Tree (arbre syntaxique abstrait) |
| HIR | High-level Intermediate Representation |
| MIR | Mid-level Intermediate Representation |
| LLVM | Backend de compilation utilisé par Rust |

## Core Explanation

### For Absolute Beginners - C'est Comme Chef Cuisinier! 👨‍🍳

Le compilateur Rust est comme un chef cuisinier: il transforme votre code (ingrédients) en exécutable (plat) avec optimisations!

C'est **exactement** comme ça fonctionne! C'est **super pratique**!

## Schéma Visuel - Chef Cuisinier

```
┌─────────────────────────────────────────┐
│  👨‍🍳 CHEF CUISINIER = Chef Cuisinier 👨‍🍳 │
├─────────────────────────────────────────┤
│                                         │
│  Concept principal                      │
│         │                                │
│         ▼ Explication                    │
│  ┌─────────────┐                        │
│  │ Chef Cuisinier │ → Fonctionne! ✅│
│  └─────────────┘                        │
│                                         │
│  Simple et puissant! ✅                 │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Chef Cuisinier" - Le compilateur Rust est comme un chef cuisinier: il transforme votre code (ingrédients) en exécutable (plat) avec optimisations!

## For Absolute Beginners

Le compilateur Rust transforme votre code source (texte) en code machine (0 et 1) que l'ordinateur peut exécuter. C'est comme un traducteur qui convertit votre recette en instructions que le robot cuisinier comprend.

**Phases de compilation :**
1. **Parsing** : Analyse le code source
2. **AST** : Crée un arbre syntaxique
3. **HIR** : Représentation de haut niveau
4. **MIR** : Représentation de niveau moyen (pour les optimisations)
5. **Code generation** : Génère le code machine

## Code Examples

### Example 1: Compilation Basique

```bash
# Compiler directement avec rustc
rustc main.rs

# Exécuter
./main  # Linux/macOS
main.exe  # Windows
```

### Example 2: Options du Compilateur

```bash
# Mode debug (défaut)
cargo build

# Mode release (optimisé)
cargo build --release

# Voir les commandes du compilateur
cargo build --verbose
```

### Example 3: Messages d'Erreur

```rust
// Code avec erreur
fn main() {
    let x = 5;
    x = 10;  // ERREUR: x n'est pas mutable
}
```

**Message d'erreur Rust :**
```
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:3:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     x = 10;
  |     ^^^^^^ cannot assign twice to immutable variable
```

## Phases de Compilation Détaillées

```
CODE SOURCE
    ↓
PARSING (analyse lexicale + syntaxique)
    ↓
AST (Abstract Syntax Tree)
    ↓
HIR (High-level IR) - résolution de noms, types
    ↓
MIR (Mid-level IR) - optimisations, borrow checking
    ↓
LLVM IR
    ↓
CODE MACHINE (binaire exécutable)
```

## Options Utiles

```bash
# Voir le code généré (MIR)
cargo rustc -- --emit mir

# Voir le code LLVM
cargo rustc -- --emit llvm-ir

# Optimisations spécifiques
RUSTFLAGS="-C opt-level=3" cargo build --release
```

## Official Resources

- [Rust Compiler Book](https://rustc-dev-guide.rust-lang.org/)
- [@official Rust Book - Compilation](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)

## Security Notes

Le compilateur Rust vérifie la sécurité à la compilation :
- Vérification des types
- Vérification de l'ownership
- Vérification des bounds
- Détection des data races
