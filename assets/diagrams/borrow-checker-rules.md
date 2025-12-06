# Borrow Checker Rules - Les 3 Règles Magiques! 

## Schéma Visuel - Les 3 Règles

```
┌─────────────────────────────────────────────────────────┐
│        BORROW CHECKER - 3 RÈGLES MAGIQUES               │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  RÈGLE 1: UN SEULE BURGER, UN PROPRIÉTAIRE 🍔          │
│  ┌─────────┐                                            │
│  │ Alice   │ possède Burger 🍔                          │
│  └─────────┘                                            │
│  ✅ OK: Un burger, un propriétaire                     │
│                                                         │
│  RÈGLE 2: MOVE = DONNER LE BURGER                       │
│  ┌─────────┐                                            │
│  │ Alice   │ donne Burger 🍔 à Bob                      │
│  └─────────┘                                            │
│         │                                               │
│         ▼ MOVE                                          │
│  ┌─────────┐                                            │
│  │ Bob     │ possède maintenant Burger 🍔               │
│  └─────────┘                                            │
│  ❌ Alice ne peut plus utiliser (donnée!)               │
│                                                         │
│  RÈGLE 3: BORROW = PRÊTER LA BURGER                     │
│  ┌─────────┐                                            │
│  │ Bob     │ prête Burger 🍔 à Charlie (&)              │
│  └─────────┘                                            │
│         │                                               │
│         ▼ BORROW                                        │
│  ┌─────────┐                                            │
│  │ Charlie │ regarde Burger 🍔 (mais ne possède pas!)   │
│  └─────────┘                                            │
│  ✅ OK: Plusieurs emprunts immuables                    │
│  ❌ PAS OK: Emprunt mutable pendant emprunt immuable    │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Mnémonique

**"Un Burger, Un Propriétaire, Move = Donner, Borrow = Prêter!"**

- **Burger** 🍔 = La valeur (mémorable!)
- **Alice, Bob, Charlie** = Personnages pour se souvenir
- **MOVE** = Donner (ownership transféré)
- **BORROW** = Prêter (référence, pas ownership)

## Règles en Résumé

1. **Une valeur, un propriétaire** (comme une burger!)
2. **MOVE** = Donner la burger (ownership transféré)
3. **BORROW** = Prêter la burger (référence, pas ownership)

