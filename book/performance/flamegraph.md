# Flamegraph - Visualiser les Performances! 🔥

## Learning Objectives

- Utiliser flamegraph pour le profiling (c'est visuel!)
- Visualiser les hotspots de performance
- Optimiser basé sur les flamegraphs
- Interpréter les résultats

## Core Explanation

### For Absolute Beginners - C'est Comme une Carte de Chaleur! 🔥

Imaginez une **carte de chaleur** 🔥:
- **Flamegraph** = Une carte qui montre où votre code passe le plus de temps
- Les zones larges = code lent
- Les zones étroites = code rapide

C'est **exactement** comme flamegraph fonctionne! C'est **super visuel**!

## Schéma Visuel - Flamegraph

```
┌─────────────────────────────────────────┐
│  🔥 FLAMEGRAPH = CARTE CHALEUR 🔥     │
├─────────────────────────────────────────┤
│                                         │
│  ┌─────────────────────┐               │
│  │ fonction_lente()    │ ← Large (lent)│
│  │ ┌───────────┐       │               │
│  │ │ sous_func │       │               │
│  │ └───────────┘       │               │
│  └─────────────────────┘               │
│                                         │
│  ┌───┐                                 │
│  │rapide│ ← Étroit (rapide)            │
│  └───┘                                 │
│                                         │
│  Large = lent, Étroit = rapide! ✅     │
│                                         │
└─────────────────────────────────────────┘
```

**Mnémonique:** "Chaleur" - Flamegraph est comme une carte de chaleur: les zones larges (chaudes) sont lentes, les zones étroites (froides) sont rapides!

## Installation

```bash
cargo install flamegraph
```

## Utilisation

```bash
# Générer un flamegraph
cargo flamegraph

# Avec options
cargo flamegraph --bin my_binary -- --my-args
```

## Interprétation

- **Largeur** : Temps d'exécution
- **Hauteur** : Stack depth
- **Couleur** : Aléatoire (pour distinction)

## Official Resources

- [flamegraph-rs](https://github.com/flamegraph-rs/flamegraph)

