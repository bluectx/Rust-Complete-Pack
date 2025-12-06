# MIRI - Détecteur de Bugs! 🕵️

## Learning Objectives

- Utiliser MIRI pour détecter UB (c'est essentiel!)
- Tester le code unsafe
- Vérifier la sécurité mémoire
- Intégrer dans CI

## Core Explanation

### For Absolute Beginners - C'est Comme un Détective! 🕵️

Imaginez un **détective** 🕵️ qui cherche des bugs:
- **MIRI** = Le détective qui trouve les problèmes de mémoire
- Il vérifie votre code unsafe
- Il trouve les bugs avant qu'ils ne causent des problèmes!

C'est **exactement** comme MIRI fonctionne! C'est **super important**!

## Schéma Visuel - MIRI

```
┌─────────────────────────────────────────┐
│  🕵️ MIRI = DÉTECTIVE 🕵️                │
├─────────────────────────────────────────┤
│                                         │
│  Code unsafe                            │
│         │                                │
│         ▼ MIRI analyse                  │
│  ┌─────────────┐                        │
│  │ Détective   │ → Trouve les bugs!    │
│  └─────────────┘                        │
│         │                                │
│         ▼ Rapport                        │
│  "Bug trouvé: use-after-free!"          │
│                                         │
│  Sécurité garantie! ✅                  │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Détective" - MIRI est comme un détective qui cherche les bugs de mémoire dans votre code unsafe!

## Installation

```bash
rustup component add miri
```

## Utilisation

```bash
# Tester avec MIRI
cargo miri test

# Exécuter avec MIRI
cargo miri run
```

## Détections

MIRI détecte :
- Use-after-free
- Double-free
- Memory leaks
- Undefined behavior
- Violations de règles unsafe

## Official Resources

- [MIRI](https://github.com/rust-lang/miri)

## Security Notes

MIRI est essentiel pour :
- Vérifier le code unsafe
- Détecter les bugs de sécurité
- Valider les invariants
- Intégrer dans CI/CD

