# Opérateurs logiques combinés

**Level:** ⭐ BEGINNER

**Objective:** Combiner && et || dans des expressions complexes

**Problem:**

Écrivez une condition combinant &&, ||, et des comparaisons. Évaluez-la pour différentes valeurs.

**Example Runs:**

```bash
$ cargo run
# Output attendu selon votre implémentation
```

**Tests:**

Run: `cargo test`

All tests must pass.

**Key Learning Points:**

- L'évaluation est court-circuitée : si && est faux au premier terme, le second n'est pas évalué.
- && (ET) a une priorité plus haute que || (OU).
