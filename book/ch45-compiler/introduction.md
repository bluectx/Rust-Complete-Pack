# Compiler Internals - Comment Rust Compile! ⚙️

## Learning Objectives

- Comprendre les phases de compilation
- Voir MIR et LLVM
- Voir des exemples COOL

## Core Explanation

### For Absolute Beginners - C'est Comme un Chef Cuisinier! 👨‍🍳

Imaginez un **chef cuisinier** qui prépare un plat:
1. **Parsing** = Lire la recette
2. **AST** = Comprendre les ingrédients
3. **MIR** = Préparer les ingrédients
4. **LLVM** = Cuire le plat
5. **Binaire** = Plat prêt!

C'est **exactement** comme le compilateur fonctionne! C'est **super intéressant**!

## Schéma Visuel - Phases de Compilation

```
┌─────────────────────────────────────────┐
│  ⚙️ COMPILATION = CHEF CUISINIER ⚙️   │
├─────────────────────────────────────────┤
│                                         │
│  Code Source (Recette)                  │
│         │                                │
│         ▼ Parsing                        │
│  AST (Ingrédients compris)               │
│         │                                │
│         ▼ MIR                            │
│  MIR (Ingrédients préparés)              │
│         │                                │
│         ▼ LLVM                           │
│  Code Machine (Plat cuisiné!)            │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Chef Cuisinier" - Le compilateur suit les étapes d'un chef: lire la recette (parsing), comprendre les ingrédients (AST), préparer (MIR), cuire (LLVM), servir (binaire)!

## Official Resources

- [Rust Compiler Book](https://rustc-dev-guide.rust-lang.org/)

